-- Your SQL goes here
create table if not exists serial_batches
(
    block_number      bigint                not null,
    transaction_index integer               not null,
    log_index         integer               not null,
    transaction_id    text                  not null,
    block_time        numeric               not null,
    batch_id          bigint                not null,
    event_type        smallint              not null,
    constraint serial_batches_batch_id_event_type_pk
    primary key(batch_id, event_type)
);