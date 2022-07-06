#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="$CONTRACT_NAME"
ROYALTY=1000

near call $CONTRACT_NAME set_contract_royalty --accountId $ACCOUNT_ID "{ \"contract_royalty\": $ROYALTY }"
