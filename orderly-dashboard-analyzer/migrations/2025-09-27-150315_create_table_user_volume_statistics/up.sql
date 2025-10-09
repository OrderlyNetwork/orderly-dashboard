-- Your SQL goes here

create table user_volume_statistics
(
    id                        bigserial not null,
    account_id                text     not null,
    broker_id                 text     not null,
    perp_volume_ytd           numeric  not null,
    perp_volume_ltd           numeric  not null,
    perp_volume_last_1_day    numeric  not null,
    perp_volume_last_7_days   numeric  not null,
    perp_volume_last_30_days  numeric  not null,
    update_time               timestamp  not null,
    constraint user_volume_statistics_uq
        primary key(account_id)
);

create index if not exists user_volume_statistics_broker_index
    on user_volume_statistics (broker_id, account_id);