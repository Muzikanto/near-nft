// Pause

#[macro_export]
macro_rules! impl_blacklist_feature {
    ($contract: ident, $instance: ident, $assert_owner: ident) => {
        use $crate::blacklist::{ContractBlacklist};

        #[near_bindgen]
        impl ContractBlacklist for $contract {
          fn is_account_blocked(&self, account_id: AccountId) -> bool {
            self.$instance.is_account_blocked(account_id)
          }

        fn block_account(&mut self, account_id: AccountId, blocked: bool) -> bool {
            self.$assert_owner();
            self.$instance.block_account(account_id, blocked)
          }
        }
    };
}
