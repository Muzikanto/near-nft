use crate::nft::{NonFungibleToken, TokenId, TokenRarity, TokenType};
use crate::nft::upgradable::NonFungibleTokenUpgradable;
use crate::nft::events_171_mf::NftUpgrade;
use near_sdk::{AccountId, env};
use near_sdk::json_types::U128;

impl NonFungibleToken {
  pub(crate) fn internal_upgrade_token(&mut self,  token_id: &TokenId, price: &U128) {
    let owner_id = self.assert_token_holder(&token_id);
    let next_rarity = self.assert_next_rarity(&token_id);

    self.internal_upgrade_token_unguarded(&owner_id, token_id, price, &next_rarity);
  }

  pub(crate) fn assert_next_rarity(&self, token_id: &TokenId) -> TokenRarity {
   let rarity = self.token_rarity_by_id.as_ref().unwrap().get(token_id).expect("Not found rarity");

    let next = match rarity {
      TokenRarity::Common => TokenRarity::Uncommon,
      TokenRarity::Uncommon => TokenRarity::Rare,
      TokenRarity::Rare => TokenRarity::Uniq,
      TokenRarity::Uniq => TokenRarity::Epic,
      TokenRarity::Epic => TokenRarity::Legendary,
      TokenRarity::Legendary => TokenRarity::Artefact,
      TokenRarity::Artefact => env::panic_str("Token fully upgraded"),
      _ => env::panic_str("Token fully upgraded"),
    };

    next
  }

  pub(crate) fn internal_upgrade_price(&self, token_type: &TokenType, rarity: &TokenRarity) -> U128 {
    let price = match token_type {
      TokenType::Armor => {
         match rarity {
          TokenRarity::Uncommon => U128::from(2000000000000000000000000),
          TokenRarity::Rare => U128::from(7000000000000000000000000),
          TokenRarity::Uniq => U128::from(24000000000000000000000000),
          TokenRarity::Epic => U128::from(81000000000000000000000000),
          TokenRarity::Legendary => U128::from(273000000000000000000000000),
          TokenRarity::Artefact => U128::from(921000000000000000000000000),
          _ => unimplemented!(),
        }
      },
      TokenType::Weapon => {
        match rarity {
          TokenRarity::Uncommon => U128::from(4000000000000000000000000),
          TokenRarity::Rare => U128::from(12000000000000000000000000),
          TokenRarity::Uniq => U128::from(40000000000000000000000000),
          TokenRarity::Epic => U128::from(135000000000000000000000000),
          TokenRarity::Legendary => U128::from(455000000000000000000000000),
          TokenRarity::Artefact => U128::from(1535000000000000000000000000),
          _ => unimplemented!(),
        }
      },
      TokenType::Shield => {
        match rarity {
          TokenRarity::Uncommon => U128::from(4000000000000000000000000),
          TokenRarity::Rare => U128::from(12000000000000000000000000),
          TokenRarity::Uniq => U128::from(40000000000000000000000000),
          TokenRarity::Epic => U128::from(135000000000000000000000000),
          TokenRarity::Legendary => U128::from(455000000000000000000000000),
          TokenRarity::Artefact => U128::from(1535000000000000000000000000),
          _ => unimplemented!(),
        }
      },
      TokenType::Pet => {
        match rarity {
          TokenRarity::Uncommon => U128::from(3000000000000000000000000),
          TokenRarity::Rare => U128::from(11000000000000000000000000),
          TokenRarity::Uniq => U128::from(36000000000000000000000000),
          TokenRarity::Epic => U128::from(121000000000000000000000000),
          TokenRarity::Legendary => U128::from(409000000000000000000000000),
          TokenRarity::Artefact => U128::from(1381000000000000000000000000),
          _ => unimplemented!(),
        }
      },
      TokenType::Jewelry => {
        match rarity {
          TokenRarity::Uncommon => U128::from(2000000000000000000000000),
          TokenRarity::Rare => U128::from(7000000000000000000000000),
          TokenRarity::Uniq => U128::from(24000000000000000000000000),
          TokenRarity::Epic => U128::from(81000000000000000000000000),
          TokenRarity::Legendary => U128::from(273000000000000000000000000),
          TokenRarity::Artefact => U128::from(921000000000000000000000000),
          _ => unimplemented!(),
        }
      },
      TokenType::Class => {
        match rarity {
          TokenRarity::Uncommon => U128::from(4000000000000000000000000),
          TokenRarity::Rare => U128::from(14000000000000000000000000),
          TokenRarity::Uniq => U128::from(48000000000000000000000000),
          TokenRarity::Epic => U128::from(162000000000000000000000000),
          TokenRarity::Legendary => U128::from(546000000000000000000000000),
          TokenRarity::Artefact => U128::from(1842000000000000000000000000),
          _ => unimplemented!(),
        }
      },
      TokenType::Race => {
        match rarity {
          TokenRarity::Uncommon => U128::from(2000000000000000000000000),
          TokenRarity::Rare => U128::from(8000000000000000000000000),
          TokenRarity::Uniq => U128::from(28000000000000000000000000),
          TokenRarity::Epic => U128::from(94000000000000000000000000),
          TokenRarity::Legendary => U128::from(318000000000000000000000000),
          TokenRarity::Artefact => U128::from(1074000000000000000000000000),
          _ => unimplemented!(),
        }
      },
      _ => {
        unimplemented!();
      }
    };

    price
  }

  pub(crate) fn internal_upgrade_token_unguarded(&mut self, owner_id: &AccountId, token_id: &TokenId, price: &U128, rarity: &TokenRarity) {
    let next_rarity = self.assert_next_rarity(&token_id);

    assert_eq!(next_rarity, rarity.clone(), "Invalid rarity upgrade");

    self.token_rarity_by_id.as_mut().unwrap().insert(&token_id, &next_rarity);

    NftUpgrade {
      owner_id: &owner_id,
      token_id: &token_id,
      rarity: &next_rarity,
      price: &price,
    }.emit();
  }
}

impl NonFungibleTokenUpgradable for NonFungibleToken {
  fn nft_upgrade(&mut self, token_id: TokenId) {
    unimplemented!();

    // self.internal_upgrade_token(&token_id, &U128::from(0));
  }
}
