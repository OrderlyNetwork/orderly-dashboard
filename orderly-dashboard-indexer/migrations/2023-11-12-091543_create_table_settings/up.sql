-- Your SQL goes here
create table if not exists settings
(
    id               integer          not null,
    value            text              not null,
    constraint setting_id
    primary key (id)
    );