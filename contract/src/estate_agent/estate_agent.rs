use crate::{Contract, ContractExt};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::collections::UnorderedMap;
use near_sdk::env;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

// created by the real estate agent
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MemorandumOfSalesAgreement {
    record_id_for_property: String,
    buyers_name: String,
    sellers_name: String,
    sellers_lawyer_name: String,
    buyers_offer_price: u128,
    additional_note: String, // additional note on the sales memoranda
    estimated_date_of_completion: String,
}

/**
 *
 * After the property has been added successfully,
 * a room for further editing is given to the real estate agent
 * he then gets a dropdown that lists all the participants that shall take
 * control of the system
 *
 * */

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RealEstateAgentProposedTransaction {
    record_id_for_property: String,
    /* participants */
    sellers_lawyer: String,     // AccountId of the sellers lawyer
    buyers_lawyer: String,      // AccountId of the buyrs lawyer
    bank: String,               // Account id of the bank
    preferred_surveyer: String, // account id of the preferred surveyor
}

// done by the estate agent...
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FormalOffer {
    pub record_id_for_property: String,
    pub buyers_full_name: String,
    sellers_full_name: String,
    sellers_address: String,      // address of the sellers
    pub type_of_mortgage: String, // AIP of full payment
}

impl MemorandumOfSalesAgreement {
    pub fn new(
        record_id_for_property: String,
        buyers_name: String,
        sellers_name: String,
        sellers_lawyer_name: String,
        buyers_offer_price: u128,
        additional_note: String,
        estimated_date_of_completion: String,
    ) -> Self {
        MemorandumOfSalesAgreement {
            record_id_for_property,
            buyers_name,
            sellers_name,
            sellers_lawyer_name, // account id of the sellers lawyer
            buyers_offer_price,
            additional_note,
            estimated_date_of_completion,
        }
    }
}

impl FormalOffer {
    pub fn new(
        record_id_for_property: String,
        buyers_full_name: String,
        sellers_full_name: String,
        sellers_address: String,
        type_of_mortgage: String,
    ) -> Self {
        FormalOffer {
            record_id_for_property,
            buyers_full_name,
            sellers_full_name,
            sellers_address,
            type_of_mortgage,
        }
    }
}

impl RealEstateAgentProposedTransaction {
    pub fn new(
        record_id_for_property: String,
        sellers_lawyer: String,
        buyers_lawyer: String,
        bank: String,
        preferred_surveyer: String,
    ) -> Self {
        RealEstateAgentProposedTransaction {
            record_id_for_property,
            sellers_lawyer,
            buyers_lawyer,
            bank,
            preferred_surveyer,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn select_proposed_transaction_participants(
        &mut self,
        record_id_for_property: String,
        sellers_lawyer: String,
        buyers_lawyer: String,
        bank: String,
        preferred_surveyer: String,
    ) {
        match self.properties.get_mut(&record_id_for_property) {
            Some(property) => {
                property.preferred_buyers_lawyer = buyers_lawyer.to_string();
                property.preferred_sellers_lawyer = sellers_lawyer.to_string();
                property.preferred_surveyer = preferred_surveyer.to_string();
                property.preferred_mortgage_institution = bank.to_string();
                property.prepare = true;
                env::log_str("preferred users updated Successfully");
            }
            None => {
                env::log_str("property not found");
            }
        }
    }

    pub fn create_formal_offer(
        &mut self,
        record_id_for_property: String,
        buyers_full_name: String,
        sellers_full_name: String,
        sellers_address: String,
        type_of_mortgage: String,
    ) {
        match self.properties.get_mut(&record_id_for_property) {
            Some(property) => {
                property.formal_offers.push(FormalOffer::new(
                    record_id_for_property.to_string(),
                    buyers_full_name.to_string(),
                    sellers_full_name.to_string(),
                    sellers_address.to_string(),
                    type_of_mortgage.to_string(),
                ));
                property.offer = true;
                env::log_str("formal offer created successfully");
            }
            None => env::log_str("property not found"),
        }
    }

    pub fn create_memorandum_of_sales_agreement(
        &mut self,
        record_id_for_property: String,
        buyers_name: String,
        sellers_name: String,
        sellers_lawyer_name: String,
        buyers_offer_price: u128,
        additional_note: String,
        estimated_date_of_completion: String,
    ) {
        match self.properties.get_mut(&record_id_for_property) {
            Some(property) => {
                property
                    .memorandum_of_sales_agreements
                    .push(MemorandumOfSalesAgreement::new(
                        record_id_for_property.to_string(),
                        buyers_name.to_string(),
                        sellers_name.to_string(),
                        sellers_lawyer_name.to_string(),
                        buyers_offer_price,
                        additional_note.to_string(),
                        estimated_date_of_completion.to_string(),
                    ));
                env::log_str("memorandum_of_sales_agreements created successfully");
            }
            None => {
                env::log_str("property not found");
            }
        }
    }

    // any document supporting the validity of the property is uploaded in this stage
    pub fn upload_valid_documents(&mut self, record_id_for_property: String, files: Vec<String>) {
        let logged_in_user = env::signer_account_id();
        match self.properties.get_mut(&record_id_for_property) {
            Some(property) => {
                if property.owners_account_id != logged_in_user.to_string() {
                    env::log_str("you are not the owner");
                } else {
                    files.into_iter().for_each(|file| {
                        property.uploaded_legal_documents.push(file.to_string());
                    });
                    env::log_str("documents uploaded sucessfully");
                }
            }
            None => {
                env::log_str("invalid property");
            }
        }
    }
}
