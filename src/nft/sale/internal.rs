use near_sdk::{AccountId, env, BorshStorageKey};
use near_sdk::collections::{UnorderedSet, LookupMap};
use crate::{SaleId};
use crate::nft::{TokenId, NonFungibleToken, JsonSale, random_use};
use near_sdk::borsh::{self, BorshSerialize};
use rand::Rng;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
  SaleTokensInner { sale_hash: Vec<u8> },
  MintCounterPerSale { sale_hash: Vec<u8> },
}

impl NonFungibleToken {
  pub(crate) fn assert_sale_not_locked(&self, sale_id: &SaleId) {
    let is_locked = self.sales_locked.as_ref().unwrap().contains(&sale_id);

    if is_locked {
      env::panic_str("Sale is locked");
    }
  }

  pub(crate) fn assert_sale_started(&self, sale_id: &SaleId) {
    let date = self.sale_date_by_id.as_ref().unwrap().get(sale_id).expect("Not found sale");
    let now = env::block_timestamp();

    if &now < date {
      env::panic_str("Sale is not started");
    }
  }

  pub(crate) fn assert_token_not_in_sale(&self, token_id: &TokenId) {
    let sale_id = self.sale_by_token.as_ref().unwrap().get(&token_id);

    if sale_id.is_some() {
      let owner_id = self.owner_by_id.get(&token_id).expect("Not found owner");

      if owner_id == self.owner_id {
        env::panic_str("Token in sale");
      }
    }
  }

  pub(crate) fn enum_get_sale(&self, sale_id: &SaleId) -> JsonSale {
    let sale = self.sale_by_id.as_ref().unwrap().get(sale_id).expect("Not found sale");
    let locked = self.sales_locked.as_ref().unwrap().contains(sale_id);
    let date = self.sale_date_by_id.as_ref().unwrap().get(sale_id);
    let ft_token = self.sale_by_ft_token.as_ref().unwrap().get(sale_id);

    let mut start_date: Option<u64> = None;
    let mut not_minted = sale.amount;
    let rand_tokens = self.sale_random_tokens.as_ref().unwrap().get(sale_id);
    if let Some(rand_tokens) = rand_tokens {
      not_minted = rand_tokens.len() as u64;
    }
    if let Some(date) = date {
      start_date = Some(date.clone());
    }

    JsonSale {
      id: sale_id.clone(),
      name: sale.name.clone(),
      price: sale.price.clone(),
      buy_max: sale.buy_max,
      per_transaction_min: sale.per_transaction_min,
      per_transaction_max: sale.per_transaction_max,
      amount: sale.amount,
      not_minted,
      locked,
      start_date,
      ft_token,
    }
  }

  pub fn internal_sale_add_token(&mut self, sale_id: &SaleId, token_id: &TokenId) {
    let sale_by_token = self.sale_by_token.as_mut().unwrap();
    let tokens_per_sale = self.sale_tokens.as_mut().unwrap();
    let random_tokens = self.sale_random_tokens.as_mut().unwrap();
    let mut sale_random = random_tokens.get(&sale_id).unwrap_or_else(|| {
      vec![]
    });

    let sale_tokens = &mut tokens_per_sale.get(&sale_id).unwrap_or_else(|| {
      UnorderedSet::new(StorageKey::SaleTokensInner {
        sale_hash: env::sha256(sale_id.as_bytes()),
      })
    });

    sale_tokens.insert(&token_id);
    tokens_per_sale.insert(&sale_id, &sale_tokens);
    sale_by_token.insert(&token_id, &sale_id);
    sale_random.push(token_id.clone());
    random_tokens.insert(&sale_id, &sale_random);

    assert_ne!(sale_tokens.len(), 0, "{}", &format!("Token does not added to sale {}", &token_id.to_string()));
  }

  pub(crate) fn internal_mint_counter_change(&mut self, owner_id: &AccountId, sale_id: &SaleId, value: &u32) {
    if let Some(mint_counter) = &mut self.sale_mint_counter {
      let mut sale_accounts = mint_counter.get(&sale_id).unwrap_or_else(|| {
        LookupMap::new(StorageKey::MintCounterPerSale {
          sale_hash: env::sha256(sale_id.as_bytes()),
        })
      });
      sale_accounts.insert(&owner_id, &value);
      mint_counter.insert(&sale_id, &sale_accounts);
    }
  }

  pub(crate) fn internal_mint_counter_by_sale(&self, owner_id: &AccountId, sale_id: &SaleId) -> u32 {
    let sale_accounts = self.sale_mint_counter.as_ref().unwrap().get(&sale_id).unwrap_or_else(||
      LookupMap::new(StorageKey::MintCounterPerSale {
        sale_hash: env::sha256(sale_id.as_bytes()),
      })
    );

    sale_accounts.get(&owner_id).unwrap_or_else(|| 0)
  }

  pub(crate) fn internal_random_tokens(&mut self, sale_id: &SaleId, amount: &u32) -> Vec<TokenId> {
    let mut random_tokens = self.sale_random_tokens.as_ref().unwrap().get(&sale_id).expect("Not found sale");

    let mut index = 0;
    let mut tokens = Vec::new();
    let mut rnd = random_use();

    loop {
      if &index == amount {
        break;
      }

      let rand_index = rnd.gen_range(0, random_tokens.len().clone());
      let token_id = random_tokens.get(rand_index).expect("Invalid token index").clone();
      let owner_id = self.owner_by_id.get(&token_id);

      if let Some(owner_id) = owner_id {
        if owner_id != self.owner_id {
          env::panic_str("Token already minted");
        }
      }

      random_tokens.remove(rand_index);
      self.sale_random_tokens.as_mut().unwrap().insert(&sale_id, &random_tokens);

      assert_eq!(&self.owner_by_id.get(&token_id).unwrap(), &self.owner_id, "Token already minted");

      tokens.push(token_id);
      index = index + 1;
    }

    tokens
  }

  pub(crate) fn internal_random_mint(&mut self, receiver_id: &AccountId, sale_id: &SaleId, amount: &u64) {
    let _amount = amount.clone() as u32;

    self.assert_sale_not_locked(&sale_id);
    self.assert_sale_started(&sale_id);

    let sale = self.sale_by_id.as_ref().unwrap().get(sale_id).expect("Not found sale");
    let buy_max = sale.buy_max;
    let per_transaction_min = sale.per_transaction_min;
    let per_transaction_max = sale.per_transaction_max;

    let rest_amount = self.sale_random_tokens.as_ref().unwrap().get(&sale_id).expect("Not found sale").len() as u32;
    let owner_minted = self.internal_mint_counter_by_sale(&receiver_id, &sale_id);

    if _amount > rest_amount {
      env::panic_str("Insufficient amount of nft");
    }
    assert!(owner_minted + _amount <= buy_max, "Mint limit");
    assert!(_amount <= per_transaction_max, "Invalid mint max amount");
    assert!(_amount >= per_transaction_min, "Invalid mint min amount");

    let tokens = self.internal_random_tokens(sale_id, &_amount);

    tokens.iter().for_each(|token_id| {
      self.internal_transfer_unguarded(&token_id, &self.owner_id.clone(), &receiver_id);
    });

    let next_minted = u32::from(owner_minted + _amount);
    self.internal_mint_counter_change(&receiver_id, sale_id, &next_minted);
  }

  // pub fn internal_nft_sale_burn(&mut self, sale_id: &SaleId) {
  //   self.assert_owner();
  //
  //   self.sale_tokens.as_mut().unwrap().remove(&sale_id);
  //   self.sale_random_tokens.as_mut().unwrap().remove(&sale_id);
  //   self.sales_locked.as_mut().unwrap().remove(&sale_id);
  //   self.sale_mint_counter.as_mut().unwrap().remove(&sale_id);
  //   self.sale_by_id.as_mut().unwrap().remove(sale_id);
  //   self.sale_date_by_id.as_mut().unwrap().remove(sale_id);
  // }
}
