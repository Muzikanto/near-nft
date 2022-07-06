use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, PromiseOrValue, env, near_bindgen};
use crate::Contract;
use near_sdk::json_types::U128;
use crate::nft::{TokenId, SaleId};

// const FT_TOKEN: AccountId = AccountId::new_unchecked("mfight-ft.testnet".to_string());

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FtTransferArgs {
  // upgradable
  pub token_id: Option<TokenId>,
  // mint
  pub sale_id: Option<SaleId>,
  pub receiver_id: Option<AccountId>,
  pub mint_amount: Option<u64>,
}

/// callbacks from FT Contracts

trait FungibleTokenReceiver {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
  fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
    let FtTransferArgs {
      token_id,
      sale_id,
      receiver_id,
      mint_amount,
    } = near_sdk::serde_json::from_str(&msg).expect("Invalid FtTransferArgs");
    let ft_token_id = env::predecessor_account_id();

    if let Some(token_id) = token_id {
      // unimplemented!();
      assert_eq!(ft_token_id, AccountId::new_unchecked("mfight-xp.testnet".to_string()), "Unavailable ft");
      assert!(amount.0 > 0, "Amount must be greater than 0");

      let owner_id = self.tokens.owner_by_id.get(&token_id).expect("Not found token");
      assert_eq!(sender_id, owner_id, "Only owner can call");

      let next_rarity = self.tokens.assert_next_rarity(&token_id);
      let token_type = self.tokens.token_type_by_id.as_mut().unwrap().get(&token_id).expect("Not found token");
      let price = self.tokens.internal_upgrade_price(&token_type,&next_rarity);

      assert_eq!(price, amount, "Invalid attached price");

      self.tokens.internal_upgrade_token_unguarded(&owner_id, &token_id, &amount, &next_rarity);

      return PromiseOrValue::Value(U128::from(0));
    }
    if let Some(sale_id) = sale_id {
      if let Some(receiver_id) = receiver_id {
        if let Some(mint_amount) = mint_amount {
          let sale = self.tokens.sale_by_id.as_ref().unwrap().get(&sale_id).expect("Not found sale");
          let ft_token = self.tokens.sale_by_ft_token.as_ref().unwrap().get(&sale_id).expect("Mint only with NEAR");

          assert_eq!(sale.price.0 * (mint_amount as u128), amount.0, "Invalid attached price");
          assert_eq!(ft_token_id, ft_token, "Unavailable ft");

          self.tokens.internal_random_mint(&receiver_id, &sale_id, &mint_amount);

          return PromiseOrValue::Value(U128::from(0));
        }
      }
    }

    env::panic_str("Invalid Args");
  }
}
