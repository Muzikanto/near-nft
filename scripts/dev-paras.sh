#!/bin/bash
source neardev/dev-account.env

TOKEN_ID="8005"
ACCOUNT_ID="muzikant.testnet"
PARAS_CONTRACT="paras-marketplace-v1.testnet"

near call $PARAS_CONTRACT storage_deposit "{\"accountId\":\"$CONTRACT_NAME\"}" --accountId $CONTRACT_NAME --depositYocto 8590000000000000000000
near call $CONTRACT_NAME nft_approve --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID\", \"account_id\": \"$PARAS_CONTRACT\",\"msg\": \"{\\\"market_type\\\":\\\"sale\\\",\\\"price\\\":\\\"1000000000000000000000000\\\",\\\"ft_token_id\\\":\\\"near\\\"}\" }"

#near call paras-marketplace-v1.testnet storage_withdraw "{\"amount\":\"8590000000000000000000\"}" --accountId $CONTRACT_NAME --depositYocto 1
