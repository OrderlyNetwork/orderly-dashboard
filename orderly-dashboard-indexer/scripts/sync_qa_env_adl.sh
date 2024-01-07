#!/bin/bash
echo "start sync trading blocks on testnet qa env #########"
echo "details: https://testnet-explorer.orderly.org/tx/0x817b2f9a4790cac68af14d53da29cf3177f52f674c2c1cc7ff3d86348bcd3115?tab=logs"
echo "start sync some deposit event block from 5148669, #########"
curl --request POST \
    --url http://127.0.0.1:8018/recovery/block \
    --header 'Content-Type: application/json' \
    --data '{ "start_block_height": 5148669, "end_block_height": 5148669}'

echo "pull all those events"
curl 'http://127.0.0.1:8018/pull_perp_trading_events?from_block=5148669&to_block=5148669'

