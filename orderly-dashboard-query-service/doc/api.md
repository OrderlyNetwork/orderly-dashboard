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

Insert test data
```shell
psql -U {your databse account} -d {your databse} -f ../../scripts/init_data.sql
```

### Daily volume
```
GET http://localhost:8089/daily_volume?from_day=2023-01-01&end_day=2024-01-01
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
      "20231214"
    ],
    "volume": [
      1950000.0,
      2120000.0
    ]
  }
}
```

### Daily trading fee
```
GET http://localhost:8089/daily_trading_fee?from_day=2023-01-01&end_day=2024-01-01
```
```json
{
  "success": true,
  "err_code": 0,
  "err_msg": null,
  "data": {
    "daytime": [
      "20231213",
      "20231214"
    ],
    "fee_amount": [
      1950000.0,
      2120000.0
    ]
  }
}
```

### Average trading count
```
GET http://localhost:8089/average_trading_count
```
```json
{
  "success": true,
  "err_code": 0,
  "err_msg": null,
  "data": {
    "latest_day_metric": 3.0,
    "latest_week_metric": 2.0,
    "latest_month_metric": 1.0
  }
}
```

### Average trading fee
```
GET http://localhost:8089/average_trading_fee
```
```json
{
  "success": true,
  "err_code": 0,
  "err_msg": null,
  "data": {
    "latest_day_metric": 3.0,
    "latest_week_metric": 2.0,
    "latest_month_metric": 1.0
  }
}
```

### Average opening count
```
GET http://localhost:8089/average_opening_count
```
```json
{
  "success": true,
  "err_code": 0,
  "err_msg": null,
  "data": {
    "latest_day_metric": 3.0,
    "latest_week_metric": 2.0,
    "latest_month_metric": 1.0
  }
}
```

### Average trading volume
```
GET http://localhost:8089/average_trading_volume
```
```json
{
  "success": true,
  "err_code": 0,
  "err_msg": null,
  "data": {
    "latest_day_metric": 3.0,
    "latest_week_metric": 2.0,
    "latest_month_metric": 1.0
  }
}
```

### Ranking of trading volume
```
GET http://localhost:8089/ranking/trading_volume?days=3&size=10
```
* days: trading volume ranking in past {days} days
* size: ranking data size

```json
{
  "success": true,
  "err_code": 0,
  "err_msg": null,
  "data": {
    "account_ids": [
      "0x8yhyjhg88iu",
      "0x8yhyjhg88i7"
    ],
    "volume": [
      1950000.0,
      2120000.0
    ]
  }
}
```

### Ranking of perp holding
```
GET http://localhost:8089/ranking/perp_holding?symbol=test&size=10
```
```json
{
  "success": true,
  "err_code": 0,
  "err_msg": null,
  "data": {
    "account_ids": [
      "0x8yhyjhg88iu",
      "0x8yhyjhg88i7"
    ],
    "holding": [
      1950000.0,
      2120000.0
    ]
  }
}
```


