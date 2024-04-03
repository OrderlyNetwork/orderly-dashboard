CREATE TABLE IF NOT EXISTS user_info
(
    account_id       text  not null,
    broker_id        text  not null,
    broker_hash        text  not null,
    address          text  not null,
    constraint pr_account_id primary key (account_id)
);-- Your SQL goes here

create index user_address on user_info (address,broker_id);
