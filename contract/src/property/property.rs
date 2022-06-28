#[allow(unused_imports)]
use near_sdk::{borsh::{self, BorshSerialize, BorshDeserialize}};

#[allow(unused_imports)]
use near_sdk::env;

#[allow(unused_imports)]
use near_sdk::near_bindgen;

#[allow(unused_imports)]
use near_sdk::serde::Serialize;

#[allow(unused_imports)]
use near_sdk::serde::Deserialize;

#[allow(unused_imports)]
use near_sdk::collections::UnorderedMap;

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Property {
	
}

impl Property {
	#[allow(dead_code)]
	pub fn new()->Self{
		Property {}
	}
}