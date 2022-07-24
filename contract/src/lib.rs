// Importing tools from the near sdk library that we'll use
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::collections::UnorderedMap;
use near_sdk::env;
// use near_sdk::json_types::{Base58PublicKey, U128};
#[allow(unused_imports)]
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

use std::collections::HashMap;

// const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
// const PROB: u8 = 128;

#[cfg(test)]
pub mod apptests;

#[allow(unused_imports)]
#[cfg(test)]
use crate::apptests::{test_mortgage, testproperty, testuser};

// adding user and property modules to the scope
pub mod estate_agent;
pub mod mortgage;
pub mod property;
pub mod user;

// importing the user & property module
#[allow(unused_imports)]
use crate::estate_agent::estate_agent::{
    FormalOffer, MemorandumOfSalesAgreement, RealEstateAgentProposedTransaction,
};

#[allow(unused_imports)]
use crate::mortgage::mortgage::ApprovalInPrinciple;
use crate::property::property::Property;
use crate::user::user::User;

pub type AccountId = String;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default, Serialize, Deserialize)]
pub struct Contract {
    owner: AccountId,
    users: HashMap<String, User>,
    properties: HashMap<String, Property>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        let users: HashMap<String, User> = HashMap::new();
        let properties: HashMap<String, Property> = HashMap::new();

        Contract {
            owner,
            users,
            properties,
        }
    }

    // registration of the new property.
    pub fn register_new_property(
        &mut self,
        record_id: String,
        owners_full_name: String,
        property_address: String,
        property_description: String,
        improvements: String,
        asking_price_from_seller: u128,
        image1: String,
        image2: String,
        image3: String,
        image4: String,
        image5: String,
        image6: String,
        other_property_attributes: Vec<String>,
    ) {
        match &self.properties.get(&record_id) {
            Some(_) => {
                env::log_str("Hash Clash try again");
            }
            None => {
                self.properties.insert(
                    record_id.to_string(),
                    Property::new(
                        record_id.to_string(),
                        owners_full_name.to_string(),
                        property_address.to_string(),
                        property_description.to_string(),
                        improvements.to_string(),
                        asking_price_from_seller,
                        image1.to_string(),
                        image2.to_string(),
                        image3.to_string(),
                        image4.to_string(),
                        image5.to_string(),
                        image6.to_string(),
                        other_property_attributes,
                    ),
                );
                env::log_str("Data Saved Successfully");
            }
        }
    }

    pub fn get_all_properties(&self) -> &HashMap<String, Property> {
        let properties = &self.properties;
        properties
    }

    pub fn show_interest_in_the_property(&mut self, property_id: String) {
        let account_id = env::signer_account_id().to_string();
        match self.properties.get_mut(&property_id) {
            Some(property_of_interest) => match self.users.get(&account_id) {
                Some(interested_user) => {
                    property_of_interest
                        .interested_customers
                        .push(interested_user.clone());
                    env::log_str("added to wishlist");
                }
                None => {
                    env::log_str("please login or update_user details");
                }
            },
            None => {
                env::log_str("invalid property");
            }
        }
    }

    pub fn count_properties(&self) -> usize {
        let properties = &self.properties.keys().len();
        *properties
    }
}
