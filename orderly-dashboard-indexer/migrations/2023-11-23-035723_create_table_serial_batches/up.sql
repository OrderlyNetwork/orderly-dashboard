-- Your SQL goes here
create table if not exists serial_batches
(
    batch_id         bigint                not null,
    transaction_id   text                  not null,
    event_type       integer               not null,
    block_time       numeric               not null,
    block_number     bigint                not null,
    constraint serial_batches_batch_id_event_type_pk
    primary key(batch_id, event_type)
);