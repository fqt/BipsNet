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

use crate::mortgage::mortgage::ApprovalInPrinciple;

use crate::{Contract, ContractExt};

pub type AccountId = String;

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub full_name: String,
    pub wallet_id: AccountId,
    pub usertype: String,
    pub organization: String,
    pub phone_number: String,
    pub approval_in_principles: Vec<ApprovalInPrinciple>,
}

impl User {
    pub fn new(
        full_name: String,
        usertype: String,
        organization: String,
        phone_number: String,
    ) -> Self {
        User {
            full_name,
            wallet_id: env::signer_account_id().to_string(),
            usertype,
            organization,
            phone_number,
            approval_in_principles: vec![],
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn update_current_user(
        &mut self,
        full_name: String,
        usertype: String,
        organization: String,
        phone_number: String,
    ) {
        self.users.insert(
            env::signer_account_id().to_string(),
            User::new(
                full_name,
                usertype,
                organization,
                phone_number,
            ),
        );
        env::log_str("You have updated your current user_details successfully");
    }
}
