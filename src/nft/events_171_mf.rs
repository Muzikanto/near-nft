use crate::event::NearEvent;
use near_sdk::AccountId;
use serde::Serialize;
use crate::nft::{TokenId, JsonSale, SaleId, Token, TokenRarity};
use near_sdk::json_types::U128;

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftCreate<'a> {
  pub token: &'a Token,
}

impl NftCreate<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftCreate<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::NftCreate(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct AccountLock<'a> {
  pub locked: &'a bool,
  pub account_id: &'a AccountId,
}

impl AccountLock<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[AccountLock<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::AccountLock(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftTransferPayout<'a> {
  pub token_id: &'a TokenId,
  pub sender_id: &'a AccountId,
  pub receiver_id: &'a AccountId,
  pub balance: &'a U128,
}

impl NftTransferPayout<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftTransferPayout<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::NftTransferPayout(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct FractionationCreate<'a> {
  pub token_id: &'a TokenId,
  pub owner_id: &'a AccountId,
}

impl FractionationCreate<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[FractionationCreate<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::FractionationCreate(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct FractionationAddToken<'a> {
  pub fractionation_id: &'a TokenId,
  pub token_id: &'a TokenId,
}

impl FractionationAddToken<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[FractionationAddToken<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::FractionationAddToken(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct FractionationComplete<'a> {
  pub token_id: &'a TokenId,
  pub from: &'a AccountId,
  pub to: &'a AccountId,
  pub completed_at: &'a u64,
}

impl FractionationComplete<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[FractionationComplete<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::FractionationComplete(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftUpgrade<'a> {
  pub owner_id: &'a AccountId,
  pub rarity: &'a TokenRarity,
  pub token_id: &'a TokenId,
  pub price: &'a U128,
}

impl NftUpgrade<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftUpgrade<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::NftUpgrade(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct SaleCreate<'a> {
  pub sale: &'a JsonSale,
}

impl SaleCreate<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[SaleCreate<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::SaleCreate(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct SaleStart<'a> {
  pub sale_id: &'a SaleId,
  pub date: &'a u64,
}

impl SaleStart<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[SaleStart<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::SaleStart(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct SaleUpdate<'a> {
  pub sale_id: &'a SaleId,
  pub date: &'a u64,
  pub per_transaction_max: &'a u32,
  pub per_transaction_min: &'a u32,
  pub buy_max: &'a u32,
}

impl SaleUpdate<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[SaleUpdate<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::SaleUpdate(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct SalePause<'a> {
  pub sale_id: &'a SaleId,
  pub pause: &'a bool,
}

impl SalePause<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[SalePause<'_>]) {
    new_171_mf_v1(Nep171MfEventKind::SalePause(data)).emit()
  }
}

// #



#[derive(Serialize, Debug)]
pub(crate) struct Nep171MfEvent<'a> {
  version: &'static str,
  #[serde(flatten)]
  event_kind: Nep171MfEventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum Nep171MfEventKind<'a> {
  NftCreate(&'a [NftCreate<'a>]),
  NftTransferPayout(&'a [NftTransferPayout<'a>]),
  FractionationCreate(&'a [FractionationCreate<'a>]),
  FractionationAddToken(&'a [FractionationAddToken<'a>]),
  FractionationComplete(&'a [FractionationComplete<'a>]),
  SaleCreate(&'a [SaleCreate<'a>]),
  SaleStart(&'a [SaleStart<'a>]),
  SaleUpdate(&'a [SaleUpdate<'a>]),
  SalePause(&'a [SalePause<'a>]),
  AccountLock(&'a [AccountLock<'a>]),
  NftUpgrade(&'a [NftUpgrade<'a>]),
}

fn new_171_mf<'a>(version: &'static str, event_kind: Nep171MfEventKind<'a>) -> NearEvent<'a> {
  NearEvent::Nep171Mf(Nep171MfEvent { version, event_kind })
}

fn new_171_mf_v1(event_kind: Nep171MfEventKind) -> NearEvent {
  new_171_mf("1.0.0", event_kind)
}

