use crate::nft::{NonFungibleToken, Token, TokenId, random_use};
use near_sdk::{AccountId, env};
use crate::nft::metadata::{TokenRarity, TokenMetadata, TokenType, TokenCollection, TokenSubType};
use crate::nft::mint::NonFungibleTokenMint;
use crate::nft::royalty::Royalty;
use crate::SaleId;
use crate::nft::events_171::NftMint;

impl NonFungibleToken {
}

impl NonFungibleTokenMint for NonFungibleToken {
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
    self.assert_owner();

    self.internal_create_nft(
      &token_id,
      Some(receiver_id.unwrap_or_else(|| self.owner_id.clone())),
      Some(token_metadata),
      rarity,
      collection,
      bind_to_owner,
      sale_id,
      perpetual_royalties,
      fractionation_id,
      token_type,
      token_sub_type,
    )
  }
}
