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
use crate::apptests::{test_mortgage, testproperty, testuser,test_insuarance};

// adding user and property modules to the scope
pub mod estate_agent;
pub mod mortgage;
pub mod property;
pub mod user;
pub mod insurance_firm;

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

    pub fn get_user_details(&self, account_id: String) -> HashMap<String, User> {
        let mut found_user = HashMap::new();
        match self.users.get(&account_id) {
            Some(user) => {
                found_user.insert(account_id.to_string(), user.clone());
                found_user
            }
            None => found_user,
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
                        record_id,
                        owners_full_name,
                        property_address,
                        property_description,
                        improvements,
                        asking_price_from_seller,
                        image1,
                        image2,
                        image3,
                        image4,
                        image5,
                        image6,
                        other_property_attributes,
                    ),
                );
                env::log_str("Data Saved Successfully");
            }
        }
    }

    pub fn get_all_properties(&self) -> &HashMap<String, Property> {
        &self.properties
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

    pub fn get_property_owner(&self, property_id: String) -> HashMap<String, User> {
        let mut user_details = HashMap::new();
        match self.properties.get(&property_id) {
            Some(property) => {
                let owner = property.owners_account_id.to_string();
                match self.users.get(&owner) {
                    Some(p_owner) => {
                        user_details.insert(property_id.to_string(), p_owner.clone());
                        user_details
                    }
                    None => {
                        env::log_str("owner not found");
                        user_details
                    }
                }
            }
            None => {
                env::log_str("property not found");
                user_details
            }
        }
    }

    pub fn get_current_user_type(&self) -> HashMap<String, User> {
        let account_id = env::signer_account_id().to_string();
        let mut stored_user: HashMap<String, User> = HashMap::new();
        match self.users.get(&account_id) {
            Some(user) => {
                stored_user.insert(account_id, user.clone());
                stored_user
            }
            None => stored_user,
        }
    }

    pub fn update_buyers_legal(
        &mut self,
        property_id: String,
        buyers_legal_account_id: String,
    ) -> String {
        match self.properties.get_mut(&property_id) {
            Some(property) => {
                property.preferred_buyers_lawyer = buyers_legal_account_id;
                env::log_str("property records updated Successfully");
                "success".to_string()
            }
            None => {
                env::log_str("property not found");
                "property_not_found".to_string()
            }
        }
    }
    pub fn update_sellers_legal(
        &mut self,
        property_id: String,
        sellers_legal_account_id: String,
    ) -> String {
        match self.properties.get_mut(&property_id) {
            Some(property) => {
                property.preferred_sellers_lawyer = sellers_legal_account_id;
                env::log_str("property records updated Successfully");
                "success".to_string()
            }
            None => {
                env::log_str("property not found");
                "property_not_found".to_string()
            }
        }
    }
    pub fn update_mortgage_institution(
        &mut self,
        property_id: String,
        mortgage_institution_account_id: String,
    ) -> String {
        match self.properties.get_mut(&property_id) {
            Some(property) => {
                property.preferred_mortgage_institution = mortgage_institution_account_id;
                env::log_str("property records updated Successfully");
                "success".to_string()
            }
            None => {
                env::log_str("property not found");
                "property_not_found".to_string()
            }
        }
    }
    pub fn update_surveyor(&mut self, property_id: String, surveyor_account_id: String) -> String {
        match self.properties.get_mut(&property_id) {
            Some(property) => {
                property.preferred_surveyer = surveyor_account_id;
                env::log_str("property records updated Successfully");
                "success".to_string()
            }
            None => {
                env::log_str("property not found");
                "property_not_found".to_string()
            }
        }
    }
}

/*
near call realestate.felabs.testnet register_new_property '{"record_id": "1234", "owners_full_name": "Felix Awere", "property_address": "Kisumu", "property_description": "Get your last chance to ... whatever",improvements: "upgrade painting", "asking_price_from_seller": 290000, "image1": "path", "image2": "path", "image3": "path", "image4": "path", "image5": "path", "image6": "path", "other_property_attributes": vec!["4 rooms", "sauna", "pool", "lovely"]}' --accountId felabs.testnet
*/
