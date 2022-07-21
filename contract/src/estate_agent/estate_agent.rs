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
    sellers_lawyer: String, // AccountId of the sellers lawyer
    buyers_lawyer: String,
    bank: String,
    tax_office: String,
    preferred_registry_officer: String,
    preferred_surveyer: String,
    preferred_settlement_organization: String,
    preferred_insurance_firm_for_property: String,
}

// done by the estate agent...
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FormalOffer {
    record_id_for_property: String,
    buyers_full_name: String,
    sellers_full_name: String,
    sellers_address: String,  // address of the sellers
    type_of_mortgage: String, // AIP of full payment
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
        tax_office: String,
        preferred_registry_officer: String,
        preferred_surveyer: String,
        preferred_settlement_organization: String,
        preferred_insurance_firm_for_property: String, // used to verify if the property is insured and if not we shall then start checking the liabilities
    ) -> Self {
        RealEstateAgentProposedTransaction {
            record_id_for_property,
            sellers_lawyer,
            buyers_lawyer,
            bank,
            tax_office,
            preferred_registry_officer,
            preferred_surveyer,
            preferred_settlement_organization,
            preferred_insurance_firm_for_property,
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
        tax_office: String,
        preferred_registry_officer: String,
        preferred_surveyer: String,
        preferred_settlement_organization: String,
        preferred_insurance_firm_for_property: String,
    ) {
        let properties = &self.properties;
        match properties.get(&record_id_for_property) {
            Some(_property) => {
                self.real_estate_proposed_transactions.insert(
                    record_id_for_property.to_string(),
                    RealEstateAgentProposedTransaction::new(
                        record_id_for_property.to_string(),
                        sellers_lawyer.to_string(),
                        buyers_lawyer.to_string(),
                        bank.to_string(),
                        tax_office.to_string(),
                        preferred_registry_officer.to_string(),
                        preferred_surveyer.to_string(),
                        preferred_settlement_organization.to_string(),
                        preferred_insurance_firm_for_property.to_string(),
                    ),
                );
                env::log_str("participants for the property have been updated successfully");
            }
            None => {
                env::log_str("property id not found");
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
        let proposed_transaction_participants = &self.real_estate_proposed_transactions;
        match proposed_transaction_participants.get(&record_id_for_property) {
            Some(_property) => {
                self.formal_offers.insert(
                    record_id_for_property.to_string(),
                    FormalOffer::new(
                        record_id_for_property.to_string(),
                        buyers_full_name.to_string(),
                        sellers_full_name.to_string(),
                        sellers_address.to_string(),
                        type_of_mortgage.to_string(),
                    ),
                );
                env::log_str("formal offer has been created successfully");
            }
            None => {
                env::log_str("you havent selected the proposed participants for this transaction");
            }
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
        let formal_offers = &self.formal_offers;
        match formal_offers.get(&record_id_for_property) {
            Some(_formal_offer) => {
                self.memorandum_of_sales_agreements.insert(
                    record_id_for_property.to_string(),
                    MemorandumOfSalesAgreement::new(
                        record_id_for_property.to_string(),
                        buyers_name.to_string(),
                        sellers_name.to_string(),
                        sellers_lawyer_name.to_string(),
                        buyers_offer_price,
                        additional_note.to_string(),
                        estimated_date_of_completion.to_string(),
                    ),
                );
                env::log_str("memorandum of sales agreement has been created successfully");
            }
            None => {
                env::log_str("formal offer has not yet been created for the property");
            }
        }
    }
}
