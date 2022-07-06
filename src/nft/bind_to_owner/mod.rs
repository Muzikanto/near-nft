pub use bind_to_owner_impl::*;

pub mod bind_to_owner_impl;
mod macros;

pub trait BindToOwnerMethods {
  fn is_bind_to_owner(&self, token_id: String) -> bool;
}
