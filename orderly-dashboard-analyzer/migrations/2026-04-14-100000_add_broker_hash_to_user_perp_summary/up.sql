alter table user_perp_summary add column if not exists broker_hash text not null default '';
alter table user_perp_summary add column if not exists address text not null default '';

create index if not exists idx_ups_nonzero_symbol_account_cover
on user_perp_summary (account_id, symbol)
include (broker_hash, address, holding)
where holding <> 0;

update user_perp_summary as ups
set broker_hash = ui.broker_hash, 
    address = ui.address
from user_info as ui
where ups.account_id = ui.account_id
  and ups.holding <> 0
  and ups.broker_hash = '';
