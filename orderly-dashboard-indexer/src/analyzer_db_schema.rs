diesel::table! {
    user_info (account_id) {
        account_id -> Text,
        broker_id -> Text,
        broker_hash -> Text,
        address -> Text,
        id -> Int8,
    }
}
