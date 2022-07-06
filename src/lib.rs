use near_sdk::{AccountId, BorshStorageKey, env, near_bindgen, PanicOnDefault, Promise, PromiseOrValue, StorageUsage};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, TreeMap, UnorderedSet};
use near_sdk::json_types::U128;

use crate::nft::*;
use crate::nft::metadata::{TokenType, NFT_METADATA_SPEC, NonFungibleTokenMetadataProvider, TokenMetadata, TokenSubType};
use std::collections::HashMap;
use crate::blacklist::BlacklistFeature;
use crate::pause::PauseFeature;
use crate::nft::royalty::RoyaltyFeature;
use crate::nft::bind_to_owner::BindToOwnerFeature;

mod nft;
mod event;
mod ft_callbacks;
mod blacklist;
mod pause;

// const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg width='24' height='24' viewBox='0 0 24 24' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M6.18169 18.0234L5.94469 17.7027V16.2629L6.32518 15.9379L5.94469 16.0618V11.0351L5.11127 11.8686C5.02928 11.9505 5.02928 12.0835 5.11127 12.1655L5.68274 12.7369V14.2216L4.97194 13.5108L5.0942 13.0884L4.62741 13.1662L4.36897 12.9078C4.20658 12.7454 4.0978 12.5497 4.04261 12.3427L4.04559 12.3362L4.03914 12.3294C3.93293 11.9127 4.04287 11.4523 4.36897 11.1263L5.05112 10.4441H5.34496V10.1503L8.50787 6.98735H6.99447V8.13528L5.94469 9.18506V7.59159L6.32518 7.38946L5.94469 7.12088V5.93757H7.79345L8.08046 6.20407L8.58016 5.93757H9.55765L9.56015 5.93506L11.0448 5.93506L11.0423 5.93757H13.006L12.1726 5.10415C12.0906 5.02216 11.9577 5.02215 11.8757 5.10415L11.3073 5.67252H9.8227L11.1334 4.36184C11.3452 4.15005 11.6136 4.02943 11.8899 4L12.845 4.29694C12.8689 4.31753 12.8922 4.33916 12.9149 4.36184L13.1867 4.63366V4.72234L13.5743 5.42901L14.0636 5.51058L16.9807 8.42764V6.98735H15.9109L14.8611 5.93757H18.0305V6.34526L17.9055 6.47762L18.0305 6.76931V13.072L18.937 12.1655C19.019 12.0835 19.019 11.9505 18.937 11.8686L18.2925 11.224V9.73941L18.6344 10.0813L18.6805 10.1933L18.8269 10.2739L19.6793 11.1263C20.1713 11.6182 20.1713 12.4158 19.6793 12.9078L19.6367 12.9504L19.1364 13.0884V13.4507L15.6135 16.9736H16.9807V15.9694L18.0305 14.9196V18.0234H15.2939L15.1472 17.8983L14.793 18.0234H10.9691L11.8757 18.9299C11.9577 19.0119 12.0906 19.0119 12.1726 18.9299L12.8181 18.2844H14.2353V18.3518L13.3438 19.2433H13.1867L12.8932 19.6934C12.8617 19.7234 12.829 19.7515 12.7953 19.7777L11.704 20C11.4949 19.9453 11.2972 19.836 11.1334 19.6722L11.015 19.5538L10.474 18.8558V19.0128L6.99447 15.5333V16.9736H8.06699L9.11677 18.0234H7.24824L6.91787 17.8983L6.68572 18.0234H6.18169ZM16.9807 9.91225L14.0558 6.98735H9.99248L6.99447 9.98536V14.0487L9.91937 16.9736H14.1289L16.9807 14.1218V9.91225Z' fill='%23FFD378'/%3E%3Cg clip-path='url(%23clip0_5878_1589)'%3E%3Cpath d='M9.51221 8.49464C10.9949 10.6511 9.70598 10.4886 9.11497 10.6715C9.70476 11.4759 9.3666 12.2318 8.43848 12.7052C9.23653 13.0257 10.2968 13.1681 9.07733 14.8593C10.287 14.1749 11.125 14.1172 11.0098 15.6672C11.8884 14.7781 13.0322 14.33 14.1725 15.0815C14.0424 13.958 14.3649 13.4659 15.4931 13.9506C14.2834 12.8408 14.7833 12.1818 15.5001 11.5362C14.2613 11.4871 14.0876 10.5406 14.1332 9.4093C13.3855 10.2228 12.8183 10.1352 12.5522 8.54342C12.0384 9.71173 11.3463 10.4574 9.5122 8.49463L9.51221 8.49464ZM11.1901 10.1567C11.4348 10.1622 11.5787 10.3422 11.717 10.5319L10.8125 12.0986C10.5021 12.1288 10.2757 12.029 10.1773 11.7319L11.0819 10.1651C11.1112 10.1601 11.1392 10.1574 11.1661 10.1568C11.1742 10.1566 11.1822 10.1566 11.1901 10.1567V10.1567ZM12.2395 10.327C12.4842 10.3323 12.6281 10.5124 12.7665 10.7021L11.7494 12.5114C11.439 12.5416 11.2127 12.4417 11.1142 12.1447L12.1313 10.3353C12.1607 10.3303 12.1887 10.3276 12.2155 10.327C12.2236 10.3268 12.2316 10.3268 12.2395 10.3269L12.2395 10.327ZM13.1318 10.8141C13.3766 10.8196 13.5205 10.9996 13.6589 11.1892L12.6564 12.976C12.346 13.0061 12.1197 12.9064 12.0213 12.6093L13.0236 10.8225C13.053 10.8175 13.081 10.8148 13.1079 10.8142C13.1159 10.814 13.124 10.814 13.1319 10.8142L13.1318 10.8141ZM13.8363 11.6537C14.0912 11.6552 14.2428 11.8358 14.3889 12.0259L13.5444 13.4887C13.24 13.5084 13.0145 13.4071 12.9092 13.122L13.7538 11.6593C13.7824 11.6554 13.8099 11.6536 13.8363 11.6538L13.8363 11.6537ZM10.0561 12.2298L11.6648 12.9522C11.7106 13.2819 11.6225 13.5418 11.331 13.6956L9.72221 12.9732C9.65369 12.6446 9.84797 12.4338 10.056 12.2298H10.0561Z' fill='%23FFD378'/%3E%3C/g%3E%3Cdefs%3E%3CclipPath id='clip0_5878_1589'%3E%3Crect width='7.7829' height='7.7829' fill='white' transform='translate(8.11719 8.09961)'/%3E%3C/clipPath%3E%3C/defs%3E%3C/svg%3E%0A";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  owner_id: AccountId,
  tokens: NonFungibleToken,
  metadata: LazyOption<NFTContractMetadata>,

  pause: PauseFeature,
  blacklist: BlacklistFeature,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
  Blacklist,

  // Nft
  NonFungibleToken,
  Metadata,
  TokenMetadata,
  Enumeration,
  Approval,
  // Nft custom
  AccountsLocked,
  NftBindToOwner,

  // Series
  SaleByToken,
  SaleTokens,
  SaleLocked,
  SaleRandomTokens,
  SaleMintCounter,
  SalesAvailable,
  SaleByFt,

  // Royalty extension
  NftRoyalty,

  // Extra info
  TokenRarity,
  TokenCollection,
  TokenType,
  TokenSubType,

  // Fractionation
  Fractionations,
  FractionationTokens,
  FractionationsIds,
  FractionationsCompleted,


  // Bind to owner extension
  BindToOwner,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new_with_default_meta(owner_id: AccountId) -> Self {
    Self::new(
      owner_id,
      NFTContractMetadata {
        spec: NFT_METADATA_SPEC.to_string(),
        name: "Mfight NFT".to_string(),
        symbol: "MFNFT".to_string(),
        icon: Some("https://images.mfight.io/near/logo.png".to_string()),
        base_uri: Option::from("https://images.mfight.io/near/nft".to_string()),
        reference: None,
        reference_hash: None,
      },
    )
  }

  #[init]
  pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
    assert!(!env::state_exists(), "Already initialized");
    metadata.assert_valid();

    let metadata = LazyOption::new(StorageKey::Metadata, Some(&metadata));
    let tokens = NonFungibleToken::new(
      StorageKey::NonFungibleToken,
      owner_id.clone(),
      Some(StorageKey::TokenMetadata),
      Some(StorageKey::Enumeration),
      Some(StorageKey::Approval),

      Some(StorageKey::SaleByToken),
      Some(StorageKey::SaleTokens),
      Some(StorageKey::SaleLocked),
      Some(StorageKey::SaleRandomTokens),
      Some(StorageKey::SaleMintCounter),
      Some(StorageKey::SalesAvailable),
      Some(StorageKey::SaleByFt),

      Some(StorageKey::TokenRarity),
      Some(StorageKey::TokenCollection),
      Some(StorageKey::TokenType),
      Some(StorageKey::TokenSubType),

      StorageKey::NftRoyalty,

      Some(StorageKey::Fractionations),
      Some(StorageKey::FractionationTokens),
      Some(StorageKey::FractionationsIds),
      Some(StorageKey::FractionationsCompleted),

     StorageKey::BindToOwner,
    );

    Self {
      owner_id: owner_id.clone(),
      tokens,
      metadata,
      pause: PauseFeature::new(),
      blacklist: BlacklistFeature::new(StorageKey::Blacklist),
    }
  }

  #[init(ignore_state)]
  #[private]
  pub fn migrate() -> Self {
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct OldNonFungibleToken {
      // owner of contract
      pub owner_id: AccountId,

      // The storage size in bytes for each new token
      pub extra_storage_in_bytes_per_token: StorageUsage,

      // always required
      pub owner_by_id: TreeMap<TokenId, AccountId>,

      // required by metadata extension
      pub token_metadata_by_id: Option<LookupMap<TokenId, TokenMetadata>>,

      // required by enumeration extension
      pub tokens_per_owner: Option<LookupMap<AccountId, UnorderedSet<TokenId>>>,

      // required by approval extension
      pub approvals_by_id: Option<LookupMap<TokenId, HashMap<AccountId, u64>>>,
      pub next_approval_id_by_id: Option<LookupMap<TokenId, u64>>,

      // ====== Sales ======
      pub sale_by_token: Option<LookupMap<TokenId, SaleId>>,
      pub sale_tokens: Option<LookupMap<SaleId, UnorderedSet<TokenId>>>,
      pub sales_locked: Option<UnorderedSet<SaleId>>,
      pub sales_available: Option<UnorderedSet<SaleId>>,
      pub sale_by_id: Option<HashMap<SaleId, Sale>>,
      pub sale_date_by_id: Option<HashMap<SaleId, u64>>,
      pub sale_random_tokens: Option<LookupMap<SaleId, Vec<TokenId>>>,
      pub sale_mint_counter: Option<LookupMap<SaleId, LookupMap<AccountId, u32>>>,
      pub sale_by_ft_token: Option<LookupMap<SaleId, AccountId>>,

      // ===== Royalty =====
      pub royalty: RoyaltyFeature,

      // ===== Lock =====
      pub bind_to_owner: BindToOwnerFeature,

      // ===== Extra =====
      pub token_rarity_by_id: Option<LookupMap<TokenId, TokenRarity>>,
      pub token_collection_by_id: Option<LookupMap<TokenId, TokenCollection>>,
      pub token_type_by_id: Option<LookupMap<TokenId, TokenType>>,
      pub token_sub_type_by_id: Option<LookupMap<TokenId, TokenSubType>>,

      // ===== Fractionation feature =====
      // key part -> new token
      pub fractionation_token_by_id: Option<LookupMap<TokenId, TokenId>>,
      pub fractionation_by_id: Option<TreeMap<TokenId, UnorderedSet<TokenId>>>,
      pub fractionation_ids: Option<UnorderedSet<TokenId>>,
      pub fractionation_completed_by_id: Option<LookupMap<TokenId, u64>>,
    }

    #[derive(BorshDeserialize)]
    struct Old {
      owner_id: AccountId,
      tokens: OldNonFungibleToken,
      metadata: LazyOption<NFTContractMetadata>,

      pause: PauseFeature,
      blacklist: BlacklistFeature,
    }

    let old: Old = env::state_read().expect("Error");

    let tokens = NonFungibleToken {
      // owner of contract
      owner_id: old.tokens.owner_id.clone(),
      // royalty_account_id: old.tokens.royalty_account_id,

      // The storage size in bytes for each new token
      extra_storage_in_bytes_per_token: old.tokens.extra_storage_in_bytes_per_token,

      // always required
      owner_by_id: old.tokens.owner_by_id,

      // required by metadata extension
      token_metadata_by_id: old.tokens.token_metadata_by_id,

      // required by enumeration extension
      tokens_per_owner: old.tokens.tokens_per_owner,

      // required by approval extension
      approvals_by_id: old.tokens.approvals_by_id,
      next_approval_id_by_id: old.tokens.next_approval_id_by_id,

      // ====== Sales ======
      sale_by_token: old.tokens.sale_by_token,
      sale_tokens: old.tokens.sale_tokens,
      sales_locked: old.tokens.sales_locked,
      sales_available: old.tokens.sales_available,
      sale_by_id: old.tokens.sale_by_id,
      sale_date_by_id: old.tokens.sale_date_by_id,
      sale_random_tokens: old.tokens.sale_random_tokens,
      sale_mint_counter: old.tokens.sale_mint_counter,
      sale_by_ft_token: old.tokens.sale_by_ft_token,

      // ===== Royalty =====
      // contract_royalty: old.tokens.contract_royalty,
      // token_royalty_by_id: old.tokens.token_royalty_by_id,
      royalty: old.tokens.royalty,

      // ===== Lock =====
      // account_locked_by_id: old.tokens.account_locked_by_id,
      // token_bind_by_id: old.tokens.token_bind_by_id,
      bind_to_owner: old.tokens.bind_to_owner,

      // ===== Extra =====
      token_rarity_by_id: old.tokens.token_rarity_by_id,
      token_collection_by_id: old.tokens.token_collection_by_id,
      token_type_by_id: old.tokens.token_type_by_id,
      token_sub_type_by_id: old.tokens.token_sub_type_by_id,

      // ===== Fractionation feature =====
      // key part -> new token
      fractionation_token_by_id: old.tokens.fractionation_token_by_id,
      fractionation_by_id: old.tokens.fractionation_by_id,
      fractionation_ids: old.tokens.fractionation_ids,
      fractionation_completed_by_id: old.tokens.fractionation_completed_by_id,
    };

    Self {
      tokens,
      owner_id: old.owner_id,
      metadata: old.metadata,
      pause: old.pause,
      blacklist: old.blacklist,
    }
  }

  pub fn assert_owner(&self) {
    assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Access Denied");
  }


  pub fn assert_caller(&self) {
    let account_id = env::predecessor_account_id();
    self.blacklist.assert_not_blocked(&account_id);
  }
  pub fn assert_transfer(&self, token_id: &TokenId, receiver_id: &AccountId) {
    self.pause.assert_not_pause();
    self.assert_caller();
  }
  pub fn assert_approve(&self, token_id: &TokenId, contract_id: &AccountId) {
    self.pause.assert_not_pause();
    self.assert_caller();
  }
  pub fn assert_burn(&self, token_id: &TokenId) {
    self.assert_caller();
    self.pause.assert_not_pause();
  }
}

impl_non_fungible_token_core!(Contract, tokens, assert_transfer);
impl_non_fungible_token_approval!(Contract, tokens, assert_approve);
impl_non_fungible_token_enumeration!(Contract, tokens);
impl_non_fungible_token_burn!(Contract, tokens, assert_burn);

impl_non_fungible_token_payout!(Contract, tokens, assert_transfer);

impl_non_fungible_token_sales!(Contract, tokens);
impl_non_fungible_token_fractionation!(Contract, tokens);
impl_non_fungible_token_mint!(Contract, tokens);

impl_bind_to_owner_feature!(Contract, tokens);
impl_royalty_feature!(Contract, tokens, assert_owner);

impl_pause_feature!(Contract, pause, assert_owner);
impl_blacklist_feature!(Contract, blacklist, assert_owner);


#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
  fn nft_metadata(&self) -> NFTContractMetadata {
    self.metadata.get().unwrap()
  }
}

