use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::near_bindgen;
use near_sdk::serde::Deserialize;
use near_sdk::serde::Serialize;

use crate::{Contract, ContractExt};

pub type AccountId = String;

// done by the mortgage agent
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ApprovalInPrinciple {
    record_id_for_property: String,
    official_full_name: AccountId,
    approved_aip_amount: u128,
    expiry_date: String,
    ea_note: String,
}

impl ApprovalInPrinciple {
    pub fn new(
        record_id_for_property: String,
        approved_aip_amount: u128,
        expiry_date: String,
        ea_note: String,
    ) -> Self {
        ApprovalInPrinciple {
            record_id_for_property,
            official_full_name: env::signer_account_id().to_string(),
            approved_aip_amount,
            expiry_date,
            ea_note,
        }
    }
}

#[near_bindgen]
impl Contract {
    /**
    * record_id_for_property,
           buyers_full_name,
           approved_aip_amount,
           expiry_date,
           ea_note,
    * */
    pub fn issue_an_approval_in_principle(
        &mut self,
        record_id_for_property: String,
        approved_aip_amount: u128,
        expiry_date: String,
        ea_note: String,
    ) {
        match self.properties.get_mut(&record_id_for_property) {
            Some(property) => {
                let formal_offers = &mut property.formal_offers;
                formal_offers.into_iter().for_each(|formal_offer| {
                    if formal_offer.record_id_for_property == record_id_for_property {
                        if formal_offer.type_of_mortgage == "AIP" {
                            match self.users.get_mut(&formal_offer.buyers_full_name) {
                                Some(user) => {
                                    user.approval_in_principles.push(ApprovalInPrinciple::new(
                                        record_id_for_property.to_string(),
                                        approved_aip_amount,
                                        expiry_date.to_string(),
                                        ea_note.to_string(),
                                    ));
                                    env::log_str("approval in principle successful")
                                }
                                None => {
                                    env::log_str("buyer not in formal offer");
                                }
                            }
                        }
                    }
                })
            }
            None => {
                env::log_str("no property found with that id");
            }
        }
    }
}
