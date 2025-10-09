-- Your SQL goes here
alter table user_volume_statistics add column IF NOT EXISTS address text not null default '';

create index if not exists user_volume_statistics_addr_index
    on user_volume_statistics (address);
