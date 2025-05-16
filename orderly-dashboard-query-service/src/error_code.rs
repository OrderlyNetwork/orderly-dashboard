pub const GENERAL_ERR: i32 = 10000;
pub const RAW_QUERY_OVERLIMIT_ERR: i32 = 10001;
pub const RAW_QUERY_EXECUTE_ERR: i32 = 10002;
pub const RAW_QUERY_BAN_ERR: i32 = 10003;
pub const QUERY_OVER_LIMIT_ERR: i32 = 10004;
pub const QUERY_OVER_EXECUTION_ERR: i32 = 10005;
pub const RAW_QUERY_OVERLIMIT_ERR_MESSAGE: &str =
    "too many raw query in parallel, please try later";
pub const RAW_QUERY_EXECUTE_ERR_MESSAGE: &str = "raw query execute failed";
pub const RAW_QUERY_BAN_ERR_MESSAGE: &str = "raw query is not open";
