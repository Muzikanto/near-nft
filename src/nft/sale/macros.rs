// Sale

#[macro_export]
macro_rules! impl_non_fungible_token_sales {
    ($contract: ident, $tokens: ident) => {
        use $crate::{SaleCore, SaleEnumeration, SaleId, JsonSale};

        #[near_bindgen]
        impl SaleCore for $contract {
          fn nft_sale_add(&mut self, id: String, name: String, amount: u64, price: U128, per_transaction_min: u32, per_transaction_max: u32, buy_max: u32, ft_token: Option<AccountId>) -> JsonSale {
            self.$tokens.nft_sale_add(id, name, amount, price, per_transaction_min, per_transaction_max, buy_max, ft_token)
          }

          fn nft_sale_start(&mut self, sale_id: SaleId, date: u64) -> JsonSale {
            self.$tokens.nft_sale_start(sale_id, date)
          }

          fn nft_sale_update(&mut self, sale_id: SaleId, date: u64, per_transaction_min: u32, per_transaction_max: u32, buy_max: u32) -> JsonSale {
            self.$tokens.nft_sale_update(sale_id, date, per_transaction_min, per_transaction_max, buy_max)
          }

           fn nft_sale_pause(&mut self, sale_id: SaleId, pause: bool) -> JsonSale {
            self.$tokens.nft_sale_pause(sale_id, pause)
          }

             #[payable]
            fn nft_buy(&mut self, receiver_id: AccountId, sale_id: SaleId, amount: u64) {
              self.$tokens.nft_buy(receiver_id, sale_id, amount)
            }
        }

          #[near_bindgen]
        impl SaleEnumeration for $contract {
          fn nft_sale_tokens(&self, sale_id: SaleId, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) -> Vec<Token> {
            self.$tokens.nft_sale_tokens(sale_id, from_index, limit)
          }

          fn nft_sales(&self) -> Vec<JsonSale> {
            self.$tokens.nft_sales()
          }

          fn nft_sale(&self, sale_id: SaleId) -> JsonSale {
            self.$tokens.nft_sale(sale_id)
          }

          fn nft_sale_not_minted(&self, sale_id: SaleId) -> u64 {
            self.$tokens.nft_sale_not_minted(sale_id)
          }

          fn nft_sales_locked(&self) -> Vec<String> {
            self.$tokens.nft_sales_locked()
          }

          fn nft_sale_token_locked(&self, token_id: TokenId) -> bool {
            self.$tokens.nft_sale_token_locked(token_id)
          }

          fn nft_sale_token_ids(&self, sale_id: SaleId, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) -> Vec<TokenId> {
             self.$tokens.nft_sale_token_ids(sale_id, from_index, limit)
          }

          fn nft_sale_account_minted(&self, sale_id: SaleId, account_id: AccountId) -> u32 {
            self.$tokens.nft_sale_account_minted(sale_id, account_id)
          }
        }
    };
}
