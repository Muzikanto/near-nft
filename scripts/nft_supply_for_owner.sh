#!/bin/bash
source neardev/account.env
ACCOUNT_ID="13b836f21a76b5c1d8cd44d9d30def58ede0b1c3045c833dfacdd629082d055e"
NEAR_ENV="$NEAR_ENV" near view $CONTRACT_NAME nft_supply_for_owner "{ \"account_id\": \"$ACCOUNT_ID\" }"
