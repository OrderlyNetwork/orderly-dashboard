use bigdecimal::{BigDecimal, FromPrimitive};
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

pub(crate) fn convert_token(amount: u128) -> anyhow::Result<BigDecimal> {
    let converted = match BigDecimal::from_u128(amount) {
        Some(converted) => converted,
        None => {
            let amount_str = amount.to_string();
            BigDecimal::from_str(&amount_str)?
        }
    };

    Ok(converted)
}
