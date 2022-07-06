#!/bin/bash
source neardev/dev-account.env
NEAR_ENV="$NEAR_ENV" near view $CONTRACT_NAME nft_royalty_value "{ }"
