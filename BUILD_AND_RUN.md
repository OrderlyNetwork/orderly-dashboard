# Build and run
## Repository structure
This Repository is organized by rust [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
It will start with a [workspace] section that will allow us to add members to the workspace by specifying the path to the package with our binary crate; in this case, that path is adder:  
Filename: [Cargo.toml](./Cargo.toml)
```toml
[workspace]
members = [
    "orderly-dashboard-indexer",
]
```
Each executable binary or library of rust need to change the top level [Cargo.toml](./Cargo.toml) to specify the `add_one` path in the members list:
```toml
[workspace]
members = [
    "orderly-dashboard-indexer",
    "add_one",
]
```
## Prerequisites
[install rust](https://www.rust-lang.org/tools/install)
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
[install postgres db](https://www.runoob.com/postgresql/mac-install-postgresql.html)  
[install diesel tool](https://diesel.rs/guides/getting-started)
```shell
cargo install diesel_cli --git https://github.com/diesel-rs/diesel.git --tag v2.1.3 --no-default-features --features "postgres"
```

```
At this point, we can build the workspace by running `cargo build --release` command. The files in your add directory should look like this:
```text
├── Cargo.lock
├── Cargo.toml
├── README.md
├── orderly-dashboard-indexer
...
└── target
```
The executable binary are here in target/release directory:
```text
tree target/release -L 1
target/release
├── ...
├── orderly-dashboard-indexer
└── ...
```

## Option1: Run without Docker
### orderly dashboard indexer
```shell
cd orderly-dashboard-indexer
```
create database on your host, put postgres url in `.env` file
```shell
echo DATABASE_URL=postgres://username:password@localhost/indexer_dbname > .env
```
execute sql in database by diesel
```shell
diesel migration run
```
run:
```shell
../target/release/orderly-dashboard-indexer -c config.example-staging.json
```
### orderly dashboard analyzer
```shell
cd orderly-dashboard-analyzer
```
create database on your host, put postgres url in `.env` file
```shell
echo DATABASE_URL=postgres://username:password@localhost/analyzer_dbname > .env
```
execute sql in database by diesel
```shell
diesel migration run
```
run: 
```shell
../target/release/orderly-dashboard-analyzer -c config-dev.json
```

### orderly dashboard query service
```shell
cd orderly-dashboard-query-service
```
create database on your host, put postgres url in `.env` file
```shell
echo INDEXER_DATABASE_URL=postgres://username:password@localhost/indexer_dbname > .env
echo ANALYZER_DATABASE_URL=postgres://username:password@localhost/analyzer_dbname >> .env
```
run: 
```shell
../target/release/orderly-dashboard-query-service -c config.example-staging.json
```

## Option2: Run with Docker
### Build images
orderly dashboard indexer:
```shell
docker build . -t orderly-dashboard-indexer -f dockerfiles/Dockerfile-indexer
```
orderly dashboard analyzer:
```shell
docker build . -t orderly-dashboard-analyzer -f dockerfiles/Dockerfile-analyzer
```
orderly dashboard query service:
```shell
docker build . -t orderly-dashboard-query-service -f dockerfiles/Dockerfile-query-service
```
orderly dashboard FE:
```shell
docker build . -t orderly-dashboard-fe -f dockerfiles/Dockerfile-FE
```

### Run
- [orderly dashboard indexer](./orderly-dashboard-indexer):
```shell
docker run --rm -it -p 8018:8018 --name orderly-dashboard-indexer -e DATABASE_URL=postgresql://[user[:password]@][netloc][:port][/indexer_dbname][?param1=value1&...] -e ORDERLY_RPC=https://rpc.orderly.network
```
- [orderly dashboard analyzer](./orderly-dashboard-analyzer)
```shell
docker run --rm -it -p 8019:8019 --name orderly-dashboard-analyzer -e DATABASE_URL=postgresql://[user[:password]@][netloc][:port][/analyzer_dbname][?param1=value1&...]
```
- [orderly dashboard query service](./orderly-dashboard-query-service)
```shell
docker run --rm -it -p 8020:8020 --name orderly-dashboard-query-service -e INDEXER_DATABASE_URL=postgresql://[user[:password]@][netloc][:port][/indexer_dbname][?param1=value1&...] -e ANALYZER_DATABASE_URL=postgresql://[user[:password]@][netloc][:port][/analyzer_dbname][?param1=value1&...]
```
- [orderly dashboard FE](./orderly-dashboard-FE)
```shell
docker run --rm -it -p 3000:3000 --name orderly-dashboard-fe -e QUERY_SERVICE_URL=https://orderly-dashboard-query-service.orderly.network -e EVM_API_URL=https://api-evm.orderly.org orderly-dashboard-fe
```