pub use pause_impl::*;

pub mod pause_impl;
mod macros;

pub trait ContractPause {
  fn is_paused(&self) -> bool;

  fn set_is_paused(&mut self, pause: bool) -> bool;
}
