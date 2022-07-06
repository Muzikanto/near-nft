// Pause

#[macro_export]
macro_rules! impl_pause_feature {
    ($contract: ident, $tokens: ident, $assert_owner:ident) => {
        use $crate::pause::{ContractPause};

        #[near_bindgen]
        impl ContractPause for $contract {
          fn is_paused(&self) -> bool {
            self.$tokens.is_paused()
          }

          fn set_is_paused(&mut self, pause: bool) -> bool {
            self.$assert_owner();
            self.$tokens.set_is_paused(pause)
          }
        }
    };
}
