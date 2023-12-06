-- Your SQL goes here

CREATE TABLE IF NOT EXISTS block_summary
(
    id                     bigint              not null,
    pulled_block_height    bigint              not null,
    pulled_block_timestamp bigint              not null,
    pulled_block_hash      text                not null,
    pulled_event_id        bigint              not null,
    pulled_spot_trade_id   bigint              not null,
    pulled_perp_trade_id   bigint              not null,
    created_time           time with time zone not null,
    updated_time           time with time zone not null,
    CONSTRAINT block_summary_pkey PRIMARY KEY (id)
);