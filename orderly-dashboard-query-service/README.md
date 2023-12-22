# orderly-dashboard-query-service
provide query api for orderly dashboard FE
## build
```shell
cargo build --release -p orderly-dashboard-query-service
```
## run
### set database url
```shell
echo ANALYZER_DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```
### start run
```shell
../target/release/orderly-dashboard-query-service -c config.example-staging.json
```
## APIs
Response format
* success: `true` | `false`
* err_code: `0` -> ok or others
* err_msg: `null` | `string`
* data: `null` | json format data
```json
{
    "data": {
    },
    "err_code": 0,
    "err_msg": null,
    "success": true
}
```
response rust data format is:
```rust
#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Response<T> {
    pub success: bool,
    pub err_code: i32,
    pub err_msg: Option<String>,
    pub data: Option<T>,
}
```
### Daily volume
request format
```
GET http://localhost:8089/daily_volume
```
result data format is
```rust
#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct DailyVolumeExtern {
    pub daytime: Vec<String>,
    pub volume: Vec<f64>,
}
```
Response
```json
{
  "success": true,
  "err_code": 0,
  "err_msg": null,
  "data": {
    "daytime": [
      "20231213",
      "20231214",
      "20231215",
      "20231216",
      "20231217",
      "20231218",
      "20231219"
    ],
    "volume": [
      1950000.0,
      2120000.0,
      2240000.0,
      2170000.0,
      1110000.0,
      2030000.0,
      2080000.0
    ]
  }
}
```
the data format is:
```rust
#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct DailyVolumeExtern {
    pub daytime: Vec<String>,
    pub volume: Vec<f64>,
}
```
### Daily volume
```
GET http://baseUrl/daily_trading_fee
```
Response of the data format is:
```rust
#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct DailyTradingFeeExtern {
    pub daytime: Vec<String>,
    pub fee_amount: Vec<f64>,
}
```