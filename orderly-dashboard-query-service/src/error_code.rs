pub const GENERAL_ERR: i32 = 10000;
pub const RAW_QUERY_OVERLIMIT_ERR: i32 = 10001;
pub const RAW_QUERY_EXECUTE_ERR: i32 = 10002;
pub const RAW_QUERY_BAN_ERR: i32 = 10003;
pub const QUERY_OVER_LIMIT_ERR: i32 = 10004;
pub const QUERY_OVER_EXECUTION_ERR: i32 = 10005;
pub const ACCOUNT_ID_CONFLICT_OR_INVALID_ERR: i32 = 10006;
pub const ACCOUNT_NOT_FOUND: i32 = 10007;
pub const DAYS_RAGE_OVER_LIMIT: i32 = 10008;
pub const SIZE_OVER_LIMIT: i32 = 10009;

pub const RAW_QUERY_OVERLIMIT_ERR_MESSAGE: &str =
    "too many raw query in parallel, please try later";
pub const RAW_QUERY_EXECUTE_ERR_MESSAGE: &str = "raw query execute failed";
pub const RAW_QUERY_BAN_ERR_MESSAGE: &str = "raw query is not open";
pub const ACCOUNT_ID_CONFLICT_OR_INVALID_ERR_MESSAGE: &str =
    "account_id and <address, broker_id> should not appear in one request not invalid";
pub const DAYS_RAGE_OVER_LIMIT_ERR_MESSAGE: &str = "days_rage_over_limit";
pub const SIZE_OVER_LIMIT_ERR_MESSAGE: &str = "size_over_limit";
