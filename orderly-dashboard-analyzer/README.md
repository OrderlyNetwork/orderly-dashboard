# orderly-dashboard-indexer
1. indexing orderly tx and decode tx params, indexing events and decode events,save those data to database
2. provide api to query blockchain data
## build
```shell
cargo build --release
```
## help
```shell
../target/release/orderly-dashboard-analyzer --help
```
```text
Usage: orderly-dashboard-indexer [OPTIONS] --config-path <CONFIG_PATH>

Options:
  -c, --config-path <CONFIG_PATH>
          
  -s, --start-block <START_BLOCK>
          
  -e, --end-block <END_BLOCK>
          
  -h, --help
          Print help
  -V, --version
          Print version
```
## run
### local run for test
We can set `start-block` and `end-block` as params to pull blocks in this range, in fact we just need to set `end-block` because we have init `start-block` in config file
```shell
../target/release/orderly-dashboard-analyzer -c config-dev.json
```
### online run
```shell
../target/release/orderly-dashboard-analyzer -c config-staging.json
```