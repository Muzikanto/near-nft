// Market Payout

#[macro_export]
macro_rules! impl_non_fungible_token_payout {
    ($contract: ident, $token: ident $(, $assert_transfer: ident)?) => {
        use $crate::{NonFungibleTokenPayout, Payout};

        #[near_bindgen]
        impl NonFungibleTokenPayout for $contract {
          #[payable]
          fn nft_transfer_payout(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: u64, balance: U128, max_len_payout: u32, memo: Option<String>) -> Payout {
              $(self.$assert_transfer(&token_id, &receiver_id);)?

              self.$token.nft_transfer_payout(receiver_id, token_id, approval_id, balance, max_len_payout, memo)
          }

          fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout {
              self.$token.nft_payout(token_id, balance, max_len_payout)
          }
        }
    };
}
