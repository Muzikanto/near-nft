use near_sdk::{AccountId, env, IntoStorageKey};
use crate::blacklist::ContractBlacklist;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BlacklistFeature {
  blocked_account_id: LookupMap<AccountId, bool>,
}

impl BlacklistFeature {
  pub fn new<Q>(prefix: Q) -> Self where
    Q: IntoStorageKey
   {
    let this = Self {
      blocked_account_id: LookupMap::new(prefix),
    };

    this
  }

  pub(crate) fn internal_block_account(&mut self, account_id: &AccountId, blocked: bool) -> bool {
    self.blocked_account_id.insert(&account_id, &blocked);

    self.internal_is_blocked(&account_id)
  }

  pub(crate) fn internal_is_blocked(&self, account_id: &AccountId) -> bool {
    self.blocked_account_id.get(&account_id).unwrap_or_else(|| false)
  }

  pub(crate) fn assert_not_blocked(&self, account_id: &AccountId) {
    let is_blocked = self.internal_is_blocked(account_id);

    if is_blocked {
      env::panic_str("Address blocked");
    }
  }
}

impl ContractBlacklist for BlacklistFeature {
  fn is_account_blocked(&self, account_id: AccountId) -> bool {
    self.internal_is_blocked(&account_id)
  }

  fn block_account(&mut self, account_id: AccountId, blocked: bool) -> bool {
    self.internal_block_account(&account_id, blocked)
  }
}
