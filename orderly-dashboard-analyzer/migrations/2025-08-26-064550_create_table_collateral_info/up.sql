-- Your SQL goes here

create table collateral_info
(
    token                     text     not null,
    token_hash                text     not null,
    decimals                  smallint not null,
    minimum_withdraw_amount   numeric  not null,
    base_weight               numeric  not null,
    discount_factor           numeric,
    haircut                   numeric  not null,
    user_max_qty              numeric  not null,
    is_collateral             boolean  not null,
    update_time               timestamp  not null,
    constraint collateral_info_uq
        primary key(token)
);