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
    buyers_full_name: AccountId,
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
            buyers_full_name: env::signer_account_id().to_string(),
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
        let properties = &self.properties;
        let approval_in_principles = &self.approval_in_principles;

        if properties.contains_key(&record_id_for_property) {
            if approval_in_principles.contains_key(&record_id_for_property) {
                env::log_str("approval in principle allready created for this property")
            } else {
                self.approval_in_principles.insert(
                    record_id_for_property.to_string(),
                    ApprovalInPrinciple::new(
                        record_id_for_property,
                        approved_aip_amount,
                        expiry_date,
                        ea_note,
                    ),
                );
            }

            env::log_str("approval in principle creation successful");
        } else {
            env::log_str("property identification not found");
        }
    }
}
