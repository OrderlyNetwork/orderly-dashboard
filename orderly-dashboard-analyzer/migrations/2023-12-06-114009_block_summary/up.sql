-- Your SQL goes here

CREATE TABLE IF NOT EXISTS block_summary
(
    id                   SERIAL PRIMARY KEY,
    latest_block_height  bigint not null,

    pulled_block_height  bigint not null,
    pulled_block_time    bigint not null,

    pulled_event_id      bigint not null,
    pulled_spot_trade_id bigint not null,
    pulled_perp_trade_id bigint not null
);