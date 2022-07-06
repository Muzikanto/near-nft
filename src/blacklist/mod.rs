pub use blacklist_impl::*;
use near_sdk::AccountId;

pub mod blacklist_impl;
mod macros;

pub trait ContractBlacklist {
  fn is_account_blocked(&self, account_id: AccountId) -> bool;

  fn block_account(&mut self, account_id: AccountId, blocked: bool) -> bool;
}
