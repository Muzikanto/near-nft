// Burn

#[macro_export]
macro_rules! impl_non_fungible_token_burn {
    ($contract: ident, $tokens: ident $(, $assert_burn: ident)?) => {
        use $crate::{NonFungibleTokenBurn};

        #[near_bindgen]
        impl NonFungibleTokenBurn for $contract {
          #[payable]
          fn nft_burn(&mut self, token_id: &TokenId) {
            $(self.$assert_burn(&token_id);)?
            self.$tokens.nft_burn(token_id)
          }
        }
    };
}
