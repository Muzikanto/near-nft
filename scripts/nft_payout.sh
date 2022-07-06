#!/bin/bash
source neardev/dev-account.env

TOKEN_ID="6136"
AMOUNT="50000000000000000000000000"

NEAR_ENV="$NEAR_ENV" near view $CONTRACT_NAME nft_payout "{ \"token_id\": \"$TOKEN_ID\", \"balance\": \"$AMOUNT\", \"max_len_payout\": 10 }"
