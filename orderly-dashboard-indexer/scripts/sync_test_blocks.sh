#!/bin/bash
echo "start sync trading blocks on testnet staging env #########"
echo "start sync some deposit event block from 1143269 #########"
curl --request POST \
    --url http://127.0.0.1:8018/recovery/block \
    --header 'Content-Type: application/json' \
    --data '{ "start_block_height": 7590217, "end_block_height": 7591837}'

echo "start sync some withdraw event from 1144102#########"
curl --request POST \
    --url http://127.0.0.1:8018/recovery/block \
    --header 'Content-Type: application/json' \
    --data '{ "start_block_height": 7590217, "end_block_height": 7591837}'

echo "start sync some withdraw events from block 1145079 #########"
curl --request POST \
    --url http://127.0.0.1:8018/recovery/block \
    --header 'Content-Type: application/json' \
    --data '{ "start_block_height": 7590217, "end_block_height": 7591837}'

# echo " settlement executions 0x4f57effc9d8096e29f597f808ddd8466f7ad49fa1aca45975a28f1bf48595823"
echo "start sync some settlement from block 1182731  #########"
curl --request POST \
    --url http://127.0.0.1:8018/recovery/block \
    --header 'Content-Type: application/json' \
    --data '{ "start_block_height": 7590217, "end_block_height": 7591837}'

# 0x2bf6e03c8564648aab39987a6d8f64f252d353755474e3a83a4d052cc8fb3213
echo "start sync some liquidation from block 2608634  #########"
curl --request POST \
    --url http://127.0.0.1:8018/recovery/block \
    --header 'Content-Type: application/json' \
    --data '{ "start_block_height": 7590217, "end_block_height": 7591837}'

echo "pull all those events"
curl 'http://127.0.0.1:8018/pull_perp_trading_events?from_block=1143269&to_block=2608634'

