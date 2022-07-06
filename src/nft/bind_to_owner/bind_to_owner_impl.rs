use near_sdk::collections::LookupMap;
use crate::bind_to_owner::BindToOwnerMethods;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::IntoStorageKey;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BindToOwnerFeature {
  pub token_bind_by_id: LookupMap<String, bool>,
}

impl BindToOwnerFeature {
  pub fn new<Q>(prefix: Q) -> Self where Q: IntoStorageKey {
    let this = Self {
      token_bind_by_id: LookupMap::new(prefix),
    };

    this
  }

  pub(crate) fn assert_bind_to_player(&self, token_id: &String) {
    let is_bind = self.internal_is_bind_to_owner(&token_id);

    assert!(
      !&is_bind,
      "Token is bind to account"
    );
  }

  pub(crate) fn internal_is_bind_to_owner(&self, token_id: &String) -> bool {
    self.token_bind_by_id.get(&token_id).unwrap_or_else(||false)
  }

  pub fn internal_token_bind_to_owner(&mut self, token_id: &String, bind_to_owner: &bool) {
    self.token_bind_by_id.insert(&token_id, &bind_to_owner);
  }
}

impl BindToOwnerMethods for BindToOwnerFeature {
  fn is_bind_to_owner(&self, token_id: String) -> bool {
    self.internal_is_bind_to_owner(&token_id)
  }
}
