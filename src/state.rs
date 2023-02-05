use crate::types::Fees;

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;


pub const COIN1: Item<Coin> = Item::new("coin1");
pub const COIN2: Item<Coin> = Item::new("coin2");

pub const OWNER: Item<Option<Addr>> = Item::new("owner");

pub const FEES: Item<Fees> = Item::new("fees");
pub const PAUSED: Item<bool> = Item::new("paused");
