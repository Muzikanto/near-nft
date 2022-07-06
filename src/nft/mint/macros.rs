// Mint

#[macro_export]
macro_rules! impl_non_fungible_token_mint {
    ($contract: ident, $tokens: ident) => {
        use $crate::{NonFungibleTokenMint};

        #[near_bindgen]
        impl NonFungibleTokenMint for $contract {
          fn nft_mint(
            &mut self,
            token_id: TokenId,
            receiver_id: Option<AccountId>,
            token_metadata: TokenMetadata,
            rarity: TokenRarity,
            collection: TokenCollection,
            token_type: TokenType,
            token_sub_type: Option<TokenSubType>,
            bind_to_owner: Option<bool>,
            sale_id: Option<SaleId>,
            perpetual_royalties: Option<Royalty>,
            fractionation_id: Option<TokenId>,
          ) -> Token {
            self.$tokens.nft_mint(
              token_id,
              receiver_id,
              token_metadata,
              rarity,
              collection,
              token_type,
              token_sub_type,
              bind_to_owner,
              sale_id,
              perpetual_royalties,
              fractionation_id,
            )
          }
        }
    };
}
