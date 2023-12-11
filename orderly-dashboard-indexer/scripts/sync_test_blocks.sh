#!/bin/bash
echo "start sync trading blocks on testnet staging env #########"
echo "start sync some deposit event block from 1143269 #########"
curl --request POST \
    --url http://127.0.0.1:8018/recovery/block \
    --header 'Content-Type: application/json' \
    --data '{ "start_block_height": 1143269, "end_block_height": 1143269}'

echo "start sync some withdraw event from 1144102#########"
curl --request POST \
    --url http://127.0.0.1:8018/recovery/block \
    --header 'Content-Type: application/json' \
    --data '{ "start_block_height": 1144102, "end_block_height": 1144102}'

echo "start sync some deposit and withdraw events from block 1145079 #########"
curl --request POST \
    --url http://127.0.0.1:8018/recovery/block \
    --header 'Content-Type: application/json' \
    --data '{ "start_block_height": 1145079, "end_block_height": 1145079}'

# echo " settlement executions 0x41f6d4bd678af00991064b9c50ec77866f3ac6fc9fd395339d341e4999df56c1"

