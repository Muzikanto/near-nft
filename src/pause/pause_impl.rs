use near_sdk::{env};
use crate::pause::ContractPause;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PauseFeature {
  paused: bool,
}

impl PauseFeature {
  pub fn new() -> Self {
    let this = Self {
      paused: false
    };

    this
  }

  pub(crate) fn internal_set_pause(&mut self, pause: &bool) -> bool {
    self.paused = pause.clone();

    self.paused.clone()
  }

  pub(crate) fn assert_not_pause(&self) {
    let is_pause = self.paused;

    if is_pause {
      env::panic_str("Contract paused");
    }
  }
}

impl ContractPause for PauseFeature {
  fn is_paused(&self) -> bool {
    self.paused
  }

  fn set_is_paused(&mut self, pause: bool) -> bool {
    self.internal_set_pause(&pause)
  }
}
