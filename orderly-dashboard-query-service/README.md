# orderly-dashboard-query-service
provide query api for orderly dashboard FE
## build
```shell
cargo build --release -p orderly-dashboard-query-service
```
## run
```shell
./target/release/orderly-dashboard-query-service
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
### Daily volume
```
GET http://baseUrl/daily_volume
```
Response
```json
{
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
            1950000,
            2120000,
            2240000,
            2170000,
            1110000,
            2030000,
            2080000
        ]
    },
    "err_code": 0,
    "err_msg": null,
    "success": true
}
```