use crate::error_code::{RAW_QUERY_EXECUTE_ERR, RAW_QUERY_OVERLIMIT_ERR};
use crate::service_base::runtime::spawn_future;
use anyhow::Context;
use bigdecimal::BigDecimal;
use diesel::types::FromSql as DieselFromSql;
use postgres::types::{FromSql, Type};
use postgres::{Client, Column, NoTls, Row};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;

const RAW_QUERY_LIMIT: u32 = 10;
const RAW_QUERY_CONTEXT: &str = "raw_query_context";
static RAW_QUERY_IN_FLIGHT: AtomicU32 = AtomicU32::new(0);
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionResult {
    pub columns: Vec<String>,
    pub data: Vec<Value>,
}

pub async fn raw_query(query_str: String) -> anyhow::Result<(i32, ExecutionResult)> {
    let value = RAW_QUERY_IN_FLIGHT.fetch_add(1, Ordering::Relaxed);
    if value >= RAW_QUERY_LIMIT {
        RAW_QUERY_IN_FLIGHT.fetch_sub(1, Ordering::Relaxed);
        return Ok((RAW_QUERY_OVERLIMIT_ERR, ExecutionResult::default()));
    }
    let (tx, rx) = oneshot::channel();
    let r = spawn_future(async move {
        let results = _raw_query(query_str).await;
        if tx.send(results).is_err() {
            tracing::warn!(target: RAW_QUERY_CONTEXT, "the receiver dropped");
        }
        Ok(())
    })
    .await;
    RAW_QUERY_IN_FLIGHT.fetch_sub(1, Ordering::Relaxed);
    if r.is_err() {
        return Ok((RAW_QUERY_EXECUTE_ERR, ExecutionResult::default()));
    }
    let results = rx.await?;
    let results = results?;
    let mut res = ExecutionResult::default();
    let mut iter = results.into_iter();
    if let Some(result) = iter.next() {
        for col in result.columns() {
            res.columns.push(col.name().to_string());
        }
        let v = postgres_row_to_json_value(result)?;
        res.data.push(v);
    }
    for raw in iter {
        let v = postgres_row_to_json_value(raw)?;
        res.data.push(v);
    }

    Ok((0, res))
}

pub async fn _raw_query(query_str: String) -> anyhow::Result<Vec<Row>> {
    let (tx, rx): (Sender<anyhow::Result<Vec<Row>>>, _) = oneshot::channel();
    let db_url = env::var("ANALYZER_DATABASE_URL")?;
    std::thread::spawn(move || {
        let client = Client::connect(&db_url, NoTls);
        if client.is_err() {
            tx.send(Err(anyhow::anyhow!("connect db failed"))).ok();
            return;
        }
        let mut client = client.unwrap();
        let prepare = client.prepare(&query_str);
        if prepare.is_err() {
            tx.send(Err(anyhow::anyhow!("prepare sql failed"))).ok();
            return;
        }
        let prepare = prepare.unwrap();

        let results = client.query(&prepare, &[]);
        if results.is_err() {
            tx.send(Err(anyhow::anyhow!("query result failed"))).ok();
            return;
        }
        let results = results.unwrap();
        tx.send(Ok(results)).ok();
    });
    let results = rx.await??;
    Ok(results)
}

pub fn postgres_row_to_json_value(row: Row) -> Result<JSONValue, Error> {
    let row_data = postgres_row_to_row_data(row)?;
    Ok(JSONValue::Object(row_data))
}

// some type-aliases I use in my project
pub type JSONValue = serde_json::Value;
pub type RowData = Map<String, JSONValue>;
pub type Error = anyhow::Error; // from: https://github.com/dtolnay/anyhow

pub fn postgres_row_to_row_data(row: Row) -> Result<RowData, Error> {
    let mut result: Map<String, JSONValue> = Map::new();
    for (i, column) in row.columns().iter().enumerate() {
        let name = column.name();
        let json_value = pg_cell_to_json_value(&row, column, i)?;
        result.insert(name.to_string(), json_value);
    }
    Ok(result)
}

pub fn pg_cell_to_json_value(
    row: &Row,
    column: &Column,
    column_i: usize,
) -> Result<JSONValue, Error> {
    let f64_to_json_number = |raw_val: f64| -> Result<JSONValue, Error> {
        let temp = serde_json::Number::from_f64(raw_val.into())
            .ok_or(anyhow::anyhow!("invalid json-float"))?;
        Ok(JSONValue::Number(temp))
    };
    Ok(match *column.type_() {
        // for rust-postgres <> postgres type-mappings: https://docs.rs/postgres/latest/postgres/types/trait.FromSql.html#types
        // for postgres types: https://www.postgresql.org/docs/7.4/datatype.html#DATATYPE-TABLE

        // single types
        Type::BOOL => get_basic(row, column, column_i, |a: bool| Ok(JSONValue::Bool(a)))?,
        Type::INT2 => get_basic(row, column, column_i, |a: i16| {
            Ok(JSONValue::Number(serde_json::Number::from(a)))
        })?,
        Type::INT4 => get_basic(row, column, column_i, |a: i32| {
            Ok(JSONValue::Number(serde_json::Number::from(a)))
        })?,
        Type::INT8 => get_basic(row, column, column_i, |a: i64| {
            Ok(JSONValue::Number(serde_json::Number::from(a)))
        })?,
        Type::TEXT | Type::VARCHAR => {
            get_basic(row, column, column_i, |a: String| Ok(JSONValue::String(a)))?
        }
        Type::JSON | Type::JSONB => get_basic(row, column, column_i, |a: JSONValue| Ok(a))?,
        Type::FLOAT4 => get_basic(row, column, column_i, |a: f32| {
            Ok(f64_to_json_number(a.into())?)
        })?,
        Type::FLOAT8 => get_basic(row, column, column_i, |a: f64| Ok(f64_to_json_number(a)?))?,
        Type::NUMERIC => get_basic(row, column, column_i, |a: Numeric| {
            Ok(JSONValue::String(a.0))
        })?,
        Type::TIMESTAMP => get_basic(row, column, column_i, |a: timestamp::WrapTimestamp| {
            Ok(JSONValue::Number(a.0.into()))
        })?,
        // these types require a custom StringCollector struct as an intermediary (see struct at bottom)
        Type::TS_VECTOR => get_basic(row, column, column_i, |a: StringCollector| {
            Ok(JSONValue::String(a.0))
        })?,

        // array types
        Type::BOOL_ARRAY => get_array(row, column, column_i, |a: bool| Ok(JSONValue::Bool(a)))?,
        Type::INT2_ARRAY => get_array(row, column, column_i, |a: i16| {
            Ok(JSONValue::Number(serde_json::Number::from(a)))
        })?,
        Type::INT4_ARRAY => get_array(row, column, column_i, |a: i32| {
            Ok(JSONValue::Number(serde_json::Number::from(a)))
        })?,
        Type::INT8_ARRAY => get_array(row, column, column_i, |a: i64| {
            Ok(JSONValue::Number(serde_json::Number::from(a)))
        })?,
        Type::TEXT_ARRAY | Type::VARCHAR_ARRAY => {
            get_array(row, column, column_i, |a: String| Ok(JSONValue::String(a)))?
        }
        Type::JSON_ARRAY | Type::JSONB_ARRAY => {
            get_array(row, column, column_i, |a: JSONValue| Ok(a))?
        }
        Type::FLOAT4_ARRAY => get_array(row, column, column_i, |a: f32| {
            Ok(f64_to_json_number(a.into())?)
        })?,
        Type::FLOAT8_ARRAY => {
            get_array(row, column, column_i, |a: f64| Ok(f64_to_json_number(a)?))?
        }
        // these types require a custom StringCollector struct as an intermediary (see struct at bottom)
        Type::TS_VECTOR_ARRAY => get_array(row, column, column_i, |a: StringCollector| {
            Ok(JSONValue::String(a.0))
        })?,

        _ => anyhow::bail!(
            "Cannot convert pg-cell \"{}\" of type \"{}\" to a JSONValue.",
            column.name(),
            column.type_().name()
        ),
    })
}

fn get_basic<'a, T: FromSql<'a>>(
    row: &'a Row,
    column: &Column,
    column_i: usize,
    val_to_json_val: impl Fn(T) -> Result<JSONValue, Error>,
) -> Result<JSONValue, Error> {
    let raw_val = row
        .try_get::<_, Option<T>>(column_i)
        .with_context(|| format!("column_name:{}", column.name()))?;
    raw_val.map_or(Ok(JSONValue::Null), val_to_json_val)
}
fn get_array<'a, T: FromSql<'a>>(
    row: &'a Row,
    column: &Column,
    column_i: usize,
    val_to_json_val: impl Fn(T) -> Result<JSONValue, Error>,
) -> Result<JSONValue, Error> {
    let raw_val_array = row
        .try_get::<_, Option<Vec<T>>>(column_i)
        .with_context(|| format!("column_name:{}", column.name()))?;
    Ok(match raw_val_array {
        Some(val_array) => {
            let mut result = vec![];
            for val in val_array {
                result.push(val_to_json_val(val)?);
            }
            JSONValue::Array(result)
        }
        None => JSONValue::Null,
    })
}

// you can remove this section if not using TS_VECTOR (or other types requiring an intermediary `FromSQL` struct)
struct StringCollector(String);
impl FromSql<'_> for StringCollector {
    fn from_sql(
        _: &Type,
        raw: &[u8],
    ) -> Result<StringCollector, Box<dyn std::error::Error + Sync + Send>> {
        let result = std::str::from_utf8(raw)?;
        Ok(StringCollector(result.to_owned()))
    }
    fn accepts(_ty: &Type) -> bool {
        true
    }
}

struct Numeric(String);
impl FromSql<'_> for Numeric {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<Numeric, Box<dyn std::error::Error + Sync + Send>> {
        let v = BigDecimal::from_sql(Some(raw)).unwrap_or_default();
        Ok(Numeric(v.to_string()))
    }
    fn accepts(_ty: &Type) -> bool {
        true
    }
}

mod timestamp {
    use diesel::data_types::PgTimestamp;
    use diesel::deserialize::FromSql as DeseFromSql;
    use diesel::pg::Pg;
    use diesel::sql_types::Timestamp;
    use postgres::types::{FromSql, Type};
    pub(crate) struct WrapTimestamp(pub i64);
    impl FromSql<'_> for WrapTimestamp {
        fn from_sql(
            _: &Type,
            raw: &[u8],
        ) -> Result<WrapTimestamp, Box<dyn std::error::Error + Sync + Send>> {
            let v: PgTimestamp = DeseFromSql::<Timestamp, Pg>::from_sql(Some(raw))
                .unwrap_or_else(|_| PgTimestamp(0));
            Ok(WrapTimestamp(v.0))
        }
        fn accepts(_ty: &Type) -> bool {
            true
        }
    }
}
