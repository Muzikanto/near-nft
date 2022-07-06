// Pause

#[macro_export]
macro_rules! impl_bind_to_owner_feature {
    ($contract: ident, $instance: ident) => {
        use $crate::bind_to_owner::{BindToOwnerMethods};

        #[near_bindgen]
        impl BindToOwnerMethods for $contract {
          fn is_bind_to_owner(&self, token_id: String) -> bool {
            self.$instance.bind_to_owner.is_bind_to_owner(token_id)
          }
        }
    };
}
