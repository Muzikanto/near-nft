use std::collections::HashMap;
use near_sdk::{AccountId, Balance, IntoStorageKey};
use near_sdk::json_types::U128;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::royalty::ContractRoyalty;
use near_sdk::collections::LookupMap;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RoyaltyFeature {
  pub amount: u32,
  pub receiver_id: AccountId,

  pub token_royalty_by_id: LookupMap<String, Royalty>,
}

pub const MINTER_ROYALTY_CAP: u32 = 2000;
pub const CONTRACT_ROYALTY_CAP: u32 = 1000;

pub type Royalty = HashMap<AccountId, u32>;

pub(crate) fn royalty_to_payout(a: u32, b: Balance) -> U128 {
  U128(a as u128 * b / 10_000u128)
}

impl RoyaltyFeature {
  pub fn new<Q>(receiver_id: AccountId, amount: u32, token_royalty_prefix: Q) -> Self
    where Q: IntoStorageKey
  {
    let this = Self {
      token_royalty_by_id: LookupMap::new(token_royalty_prefix),
      amount,
      receiver_id,
    };

    this
  }

  pub(crate) fn internal_royalty_calculate(&self, perpetual_royalties: Option<Royalty>) -> HashMap<AccountId, u32> {
    let mut royalty = HashMap::new();
    let mut total_perpetual = 0;

    // user added perpetual_royalties (percentage paid with every transfer)
    if let Some(perpetual_royalties) = perpetual_royalties {
      assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");
      for (account, amount) in perpetual_royalties {
        royalty.insert(account, amount);
        total_perpetual += amount;
      }
    }

    assert!(total_perpetual <= MINTER_ROYALTY_CAP, "Perpetual royalties cannot be more than 20%");

    royalty

  }
}

impl ContractRoyalty for RoyaltyFeature {
  fn set_royalty_value(&mut self, contract_royalty: u32) {
    assert!(contract_royalty <= CONTRACT_ROYALTY_CAP, "Contract royalties limited to 10% for owner");

    self.amount = contract_royalty;
  }

  fn set_royalty_account(&mut self, account_id: AccountId) -> AccountId {
    self.receiver_id = account_id.clone();
    self.receiver_id.clone()
  }

  fn nft_royalty_value(&self) -> u32 {
    self.amount
  }

  fn nft_royalty_account(&self) -> AccountId {
    self.receiver_id.clone()
  }
}
