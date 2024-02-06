pub mod gas_consumption;
pub mod symbols_config;
pub mod trading_events;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, PartialEq)]
pub enum Response<T> {
    Success(SuccessResponse<T>),
    Failure(FailureResponse),
}

impl<T: Default> Response<T> {
    pub fn empty_success() -> Self {
        Self::Success(SuccessResponse::default())
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Response<T> {
    fn deserialize<D>(deserializer: D) -> Result<Response<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = Map::deserialize(deserializer)?;

        let success = map
            .remove("success")
            .ok_or_else(|| de::Error::missing_field("success"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;
        let rest = Value::Object(map);

        if success {
            SuccessResponse::deserialize(rest)
                .map(Response::Success)
                .map_err(de::Error::custom)
        } else {
            FailureResponse::deserialize(rest)
                .map(Response::Failure)
                .map_err(de::Error::custom)
        }
    }
}

impl<T: Serialize> Serialize for Response<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Response::Success(success) => {
                ser::SuccessResponse::new(&success.data).serialize(serializer)
            }
            Response::Failure(failure) => {
                ser::FailureResponse::new(failure.code, &failure.message).serialize(serializer)
            }
        }
    }
}

mod ser {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct SuccessResponse<'a, T> {
        success: bool,
        data: &'a T,
    }

    impl<'a, T> SuccessResponse<'a, T> {
        pub fn new(data: &'a T) -> Self {
            Self {
                success: true,
                data,
            }
        }
    }

    #[derive(Serialize)]
    pub struct FailureResponse<'a> {
        success: bool,
        code: i32,
        message: &'a str,
    }

    impl<'a> FailureResponse<'a> {
        pub fn new(code: i32, message: &'a str) -> Self {
            Self {
                success: false,
                code,
                message,
            }
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct SuccessResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data: Some(data) }
    }

    #[allow(dead_code)]
    pub fn into_data(self) -> Option<T> {
        self.data
    }

    #[allow(dead_code)]
    pub fn as_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FailureResponse {
    code: i32,
    message: String,
}

impl FailureResponse {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    #[allow(dead_code)]
    pub fn batch_ids_too_long() -> Self {
        Self {
            code: -1103,
            message: "Batch IDs too long".to_owned(),
        }
    }

    #[allow(dead_code)]
    pub fn err_code(&self) -> i32 {
        self.code
    }
}

impl Default for FailureResponse {
    fn default() -> Self {
        Self {
            code: 0,
            message: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct RecoveryBlockRequest {
    pub start_block_height: u64,
    pub end_block_height: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NetworkInfo {
    pub finalized_height: Option<u64>,
}
