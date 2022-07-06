#!/bin/bash
source neardev/dev-account.env
SALE_ID="badge_1"
PER_TX_MIN=1
PER_TX_MAX=1
BUY_MAX=1

near call $CONTRACT_NAME nft_sale_update --accountId $CONTRACT_NAME "{ \"sale_id\": \"$SALE_ID\", \"date\": 1643280497042, \"per_transaction_min\": $PER_TX_MIN, \"per_transaction_max\": $PER_TX_MAX, \"buy_max\": $BUY_MAX }"
