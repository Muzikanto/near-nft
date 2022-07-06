// Fractionation

#[macro_export]
macro_rules! impl_non_fungible_token_fractionation {
    ($contract: ident, $tokens: ident) => {
        use $crate::{NonFungibleTokenFractionation, Fractionation};

        #[near_bindgen]
        impl NonFungibleTokenFractionation for $contract {
          fn nft_fractionation(&self, token_id: TokenId) -> Fractionation {
            self.$tokens.nft_fractionation(token_id)
          }
          fn nft_fractionations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Fractionation> {
            self.$tokens.nft_fractionations(from_index, limit)
          }
          fn nft_fractionations_supply(&self) -> U128 {
            self.$tokens.nft_fractionations_supply()
          }
          fn nft_fractionation_complete(&mut self, token_id: TokenId) {
            self.$tokens.nft_fractionation_complete(token_id)
          }
        }
    };
}

// Upgradable

#[macro_export]
macro_rules! impl_non_fungible_token_upgradable {
    ($contract: ident, $tokens: ident) => {
        use $crate::{NonFungibleTokenUpgradable};

        #[near_bindgen]
        impl NonFungibleTokenUpgradable for $contract {
          fn nft_upgrade(&mut self, token_id: TokenId) -> bool {
            self.$tokens.nft_upgrade(token_id)
          }
        }
    };
}
