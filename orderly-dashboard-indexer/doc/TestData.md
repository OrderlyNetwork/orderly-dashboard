# TestData
## Acquired test data from staging
include: deposit,withdraw,perp trade,settlement,liquidation
```shell
./scripts/sync_test_blocks.sh
```

## Acquired test data from qa
include: adl  
before run script, you need to create a database to for qa env's data and update local `.env` files:
```shell
# DATABASE_URL=postgres://username:password@localhost/diesel_demo
DATABASE_URL=postgres://username:password@localhost/diesel_demo_qa
```
and then run script:
```shell
./scripts/sync_qa_env_adl.sh
```