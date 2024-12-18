use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::types::{H160, H256, I256, U256};
use std::str::FromStr;

#[macro_export]
macro_rules! await_retry_or_error {
    ($query: expr, $number_of_retries: expr, $error_message: expr) => {{
        let mut interval = crate::settings::INTERVAL;
        let mut retry_attempt = 0usize;
        loop {
            if retry_attempt == $number_of_retries {
                return Err(anyhow::anyhow!(
                    "Failed to perform query to CeFi after {} attempts. Stop trying.",
                    $number_of_retries
                ));
            }
            retry_attempt += 1;

            match $query.await {
                Ok(res) => break Ok(res),
                Err(async_error) => {
                    tracing::error!(
                        target: crate::ORDERLY_DASHBOARD_INDEXER,
                        "Error occurred during {}: \n{:?} \n Retrying in {} milliseconds...",
                        async_error,
                        &$error_message,
                        interval.as_millis(),
                    );
                    tokio::time::sleep(interval).await;
                    if interval < crate::settings::MAX_DELAY_TIME {
                        interval *= 2;
                    }
                }
            }
        }
    }};
}

pub fn to_hex_format(bytes: &[u8]) -> String {
    "0x".to_string() + &hex::encode(bytes)
}

pub fn convert_amount(amount: i128) -> anyhow::Result<BigDecimal> {
    let converted = match BigDecimal::from_i128(amount) {
        Some(converted) => converted,
        None => {
            let amount_str = amount.to_string();
            BigDecimal::from_str(&amount_str)?
        }
    };

    Ok(converted)
}

pub fn format_hash(hash: H256) -> String {
    format!("{:?}", hash)
}

pub fn format_hash_160(hash: H160) -> String {
    format!("{:?}", hash)
}

pub fn u256_to_i128(n: U256) -> i128 {
    I256::from_raw(n).as_i128()
}

pub fn hex_bytes(bytes: &[u8]) -> String {
    "0x".to_string() + &hex::encode(bytes)
}
