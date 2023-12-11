# Orderly dashboard
Orderly Dashboard is aim to be a web-based data analyze platform that will present some of the most popular analytical indicator data in Orderly Blockchain. It also allows users to query the analyzed data of transactions that have occurred in Orderly Blockchain. Users can also query the analyzed data through the Analyzer module of Orderly Dashboard. Analyzed  data can be displayed in the form of visual charts. The statistical analysis data can be downloaded by data users. Orderly Dashboard will generate chart links from data,those generated chart links can be also embedded into other web pages or shared by users in other ways. Users can also construct their own queries through the data of Orderly Dashboard and create their own Dashboard Board by Orderly Dashboard data.
## Components
* [Order dashboard indexer](./orderly-dashboard-indexer)
  * indexing orderly blockchain's tx and events and decode them, save them to database, provide api to query them by sequence
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
create database on your host, put url in `.env` file
```shell
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```
create table
```shell
diesel migration generate table_name
```
execute sql in database by diesel
```shell
diesel migration run
```
At this point, we can build the workspace by running `cargo build --release` command. The files in your add directory should look like this:
```text
├── Cargo.lock
├── Cargo.toml
├── README.md
├── orderly-dashboard-indexer
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
## Tools
### explorer
* testnet: https://testnet-explorer.orderly.org/
* mainnet: https://explorer.orderly.network