#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="$CONTRACT_NAME"
ROYALTY_ACCOUNT="mfight.testnet"

near call $CONTRACT_NAME set_royalty_account --accountId $ACCOUNT_ID "{ \"account_id\": $ROYALTY_ACCOUNT }"
