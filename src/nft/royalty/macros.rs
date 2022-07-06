// Pause

#[macro_export]
macro_rules! impl_royalty_feature {
    ($contract: ident, $instance: ident, $assert_owner: ident) => {
        use $crate::royalty::{ContractRoyalty};

        #[near_bindgen]
        impl ContractRoyalty for $contract {
          fn set_royalty_value(&mut self, contract_royalty: u32) {
            self.$assert_owner();
            self.$instance.royalty.set_royalty_value(contract_royalty)
          }
          fn set_royalty_account(&mut self, account_id: AccountId) -> AccountId {
            self.$assert_owner();
            self.$instance.royalty.set_royalty_account(account_id)
          }

          fn nft_royalty_value(&self) -> u32 {
            self.$instance.royalty.nft_royalty_value()
          }
          fn nft_royalty_account(&self) -> AccountId {
            self.$instance.royalty.nft_royalty_account()
          }
        }
    };
}
