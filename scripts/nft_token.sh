#!/bin/bash
source neardev/account.env

TOKEN_ID="100083"

NEAR_ENV="$NEAR_ENV" near view $CONTRACT_NAME nft_token "{ \"token_id\": \"$TOKEN_ID\" }"
