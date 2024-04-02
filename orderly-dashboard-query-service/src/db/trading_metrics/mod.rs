use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::sql_types::{Date, Numeric, Timestamp};
use diesel::QueryableByName;

use orderly_dashboard_analyzer::db::block_summary::find_block_summary;
#[allow(unused_imports)]
use orderly_dashboard_analyzer::{
    db::{hourly_orderly_perp::HourlyOrderlyPerp, POOL},
    schema::hourly_orderly_perp,
    schema::hourly_orderly_perp::dsl::*,
};

use crate::format_extern::trading_metrics::{DailyTradingFeeExtern, DailyVolumeExtern};

pub mod average;
pub mod orderly_daily_perp;
pub mod orderly_daily_token;
pub mod ranking;

#[derive(Debug, Clone, QueryableByName)]
pub struct DailyVolume {
    #[sql_type = "Date"]
    trading_day: NaiveDate,
    #[sql_type = "Numeric"]
    trading_volume: BigDecimal,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct DailyFee {
    #[sql_type = "Date"]
    trading_day: NaiveDate,
    #[sql_type = "Numeric"]
    trading_fee: BigDecimal,
}

pub async fn get_block_height() -> i64 {
    #[allow(unused_imports)]
    use orderly_dashboard_analyzer::{
        db::{block_summary::BlockSummary, POOL},
        schema::block_summary,
        schema::block_summary::dsl::*,
    };

    let block = find_block_summary("trade".to_string()).await.unwrap();
    block.pulled_block_height
}

pub async fn get_daily_volume(
    from_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> DailyVolumeExtern {
    let sql_query = diesel::sql_query(
        "select \
      date(block_hour) as trading_day,\
      sum(trading_volume) as trading_volume \
      from hourly_orderly_perp where block_hour>=$1 and block_hour<=$2 \
      group by trading_day order by trading_day asc;",
    );

    let result: Result<Vec<DailyVolume>, _> = sql_query
        .bind::<Timestamp, _>(from_time)
        .bind::<Timestamp, _>(end_time)
        .get_results_async::<DailyVolume>(&POOL)
        .await;

    return match result {
        Ok(daily_volumes) => {
            let mut daytime_vec: Vec<String> = Vec::new();
            let mut volume_vec: Vec<f64> = Vec::new();
            let daily_format = "%Y-%m-%d";

            for volume in daily_volumes {
                daytime_vec.push(volume.trading_day.format(daily_format).to_string());
                volume_vec.push(volume.trading_volume.to_f64().unwrap());
            }

            DailyVolumeExtern {
                daytime: daytime_vec,
                volume: volume_vec,
            }
        }
        Err(error) => {
            println!("{}", error);
            DailyVolumeExtern {
                daytime: vec![],
                volume: vec![],
            }
        }
    };
}

pub async fn get_daily_trading_fee(
    from_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> DailyTradingFeeExtern {
    let query = diesel::sql_query(
        "select \
      date(block_hour) as trading_day,\
      sum(trading_fee) as trading_fee \
      from hourly_orderly_perp where block_hour>=$1 and block_hour<=$2 \
      group by trading_day order by trading_day asc;",
    );

    let result: Result<Vec<DailyFee>, _> = query
        .bind::<Timestamp, _>(from_time)
        .bind::<Timestamp, _>(end_time)
        .get_results_async::<DailyFee>(&POOL)
        .await;

    return match result {
        Ok(daily_fees) => {
            let mut daytime_vec: Vec<String> = Vec::new();
            let mut volume_vec: Vec<f64> = Vec::new();
            let daily_format = "%Y-%m-%d";

            for daily_fee in daily_fees {
                daytime_vec.push(daily_fee.trading_day.format(daily_format).to_string());
                volume_vec.push(daily_fee.trading_fee.to_f64().unwrap());
            }
            DailyTradingFeeExtern {
                daytime: daytime_vec,
                fee_amount: volume_vec,
            }
        }
        Err(error) => {
            println!("{}", error);
            DailyTradingFeeExtern {
                daytime: vec![],
                fee_amount: vec![],
            }
        }
    };
}
