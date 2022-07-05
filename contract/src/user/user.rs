#[allow(unused_imports)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

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

pub type AccountId = String;

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    full_name: String,
    wallet_id: AccountId,
    usertype: String,
    organization: String,
}

impl User {
    pub fn new(full_name: String, usertype: String, organization: String) -> Self {
        User {
            full_name,
            wallet_id: env::signer_account_id().to_string(),
            usertype,
            organization,
        }
    }
}
