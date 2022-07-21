// Importing tools from the near sdk library that we'll use
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::collections::UnorderedMap;
use near_sdk::env;
#[allow(unused_imports)]
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::collections::HashMap;

#[cfg(test)]
mod apptests;

#[allow(unused_imports)]
#[cfg(test)]
use crate::apptests::{test_mortgage, testproperty, testuser};

// adding user and property modules to the scope
mod estate_agent;
mod mortgage;
mod property;
mod user;

// importing the user & property module
use crate::estate_agent::estate_agent::{
    FormalOffer, MemorandumOfSalesAgreement, RealEstateAgentProposedTransaction,
};
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
    approval_in_principles: HashMap<String, ApprovalInPrinciple>,
    real_estate_proposed_transactions: HashMap<String, RealEstateAgentProposedTransaction>,
    formal_offers: HashMap<String, FormalOffer>,
    memorandum_of_sales_agreements: HashMap<String, MemorandumOfSalesAgreement>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        let users: HashMap<String, User> = HashMap::new();
        let properties: HashMap<String, Property> = HashMap::new();
        let approval_in_principles: HashMap<String, ApprovalInPrinciple> = HashMap::new();
        let real_estate_proposed_transactions: HashMap<String, RealEstateAgentProposedTransaction> =
            HashMap::new();
        let formal_offers: HashMap<String, FormalOffer> = HashMap::new();
        let memorandum_of_sales_agreements: HashMap<String, MemorandumOfSalesAgreement> =
            HashMap::new();

        Contract {
            owner,
            users,
            properties,
            approval_in_principles,
            real_estate_proposed_transactions,
            formal_offers,
            memorandum_of_sales_agreements,
        }
    }

    pub fn register_new_portal_user(
        &mut self,
        full_name: String,
        usertype: String,
        organization: String,
    ) {
        // for now we assume that every user taking part in the system should at least have a near wallet id
        self.users.insert(
            env::signer_account_id().to_string(),
            User::new(
                full_name.to_string(),
                usertype.to_string(),
                organization.to_string(),
            ),
        );
        env::log_str("user details updated Successfully");
    }

    // registration of the new property.
    pub fn register_new_property(
        &mut self,
        owners_full_name: String,
        property_address: String,
        property_description: String,
        improvements: String,
        asking_price_from_seller: u128,
        energy_certificate: String,
        floor_plan: String,
        gas_certificate: String,
        environmental_assesment: String,
        image1: String,
        image2: String,
        image3: String,
        image4: String,
        image5: String,
        image6: String,
    ) {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect();
        match &self.properties.get(&rand_string) {
            Some(_) => {
                env::log_str("Hash Clash try again");
            }
            None => {
                self.properties.insert(
                    rand_string.to_string(),
                    Property::new(
                        rand_string.to_string(),
                        owners_full_name.to_string(),
                        property_address.to_string(),
                        property_description.to_string(),
                        improvements.to_string(),
                        asking_price_from_seller,
                        energy_certificate.to_string(),
                        floor_plan.to_string(),
                        gas_certificate.to_string(),
                        environmental_assesment.to_string(),
                        image1.to_string(),
                        image2.to_string(),
                        image3.to_string(),
                        image4.to_string(),
                        image5.to_string(),
                        image6.to_string(),
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

    pub fn count_properties(&self) -> usize {
        let properties = &self.properties.keys().len();
        *properties
    }
}

// property_address: String, property_description: String,improvements: String,asking_price_from_seller: u128,energy_certificate: String,floor_plan: String,gas_certificate: String,environmental_assesment: String,image1: String,image2: String,image3: String,image4: String,image5: String,image6: String,

// near call realestate.felabs.testnet register_new_property '{"owners_full_name": "Felix", "property_address": "Kisumu", "property_description": "lorem ipsum dolor sit amet consectetur ", "improvements": "none","asking_price_from_seller": 1800,"energy_certificate": "Strig","floor_plan": "String","gas_certificate": "String","environmental_assesment": "String","image1": "String","image2": "String","image3": "String","image4": "String","image5": "String","image6": "String"}' --accountId felabs1.testnet
