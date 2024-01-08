# TestData
## Acquired test data from staging
include: deposit,withdraw,perp trade,settlement,liquidation  
<b>Terminal 1</b>
```shell
../target/release/orderly-dashboard-indexer -c config.example-staging.jso
```
<b>Terminal 2</b>
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
<b>Terminal 1</b>
```shell
../target/release/orderly-dashboard-indexer -c config.example-qa.json
```
<b>Terminal 2</b>
```shell
./scripts/sync_qa_env_adl.sh
```