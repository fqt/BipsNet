use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::serde::Deserialize;
use near_sdk::serde::Serialize;

// use crate::{Contract, ContractExt};

// const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
// const PROB: u8 = 128;

#[cfg(test)]
// mod apptests;
#[allow(unused_imports)]
#[cfg(test)]
use crate::apptests::{test_mortgage, testproperty, testuser};

// importing the user & property module
use crate::estate_agent::estate_agent::{
    FormalOffer, MemorandumOfSalesAgreement, RealEstateAgentProposedTransaction,
};
use crate::mortgage::mortgage::ApprovalInPrinciple;
use crate::user::user::User;

pub type AccountId = String;

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
// files stored in arweave
pub struct Property {
    record_id: String,
    owners_full_name: String,
    pub owners_account_id: AccountId,
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
    pub interested_customers: Vec<User>,
    pub prepare: bool,
    pub offer: bool,
    pub due_dilligence: bool,
    pub exchange: bool,
    pub completion: bool,
    pub validated: bool,
    pub other_property_attributes: Vec<String>,
    pub approval_in_principles: Vec<ApprovalInPrinciple>,
    pub real_estate_proposed_transactions: Vec<RealEstateAgentProposedTransaction>,
    pub formal_offers: Vec<FormalOffer>,
    pub memorandum_of_sales_agreements: Vec<MemorandumOfSalesAgreement>,
    pub preferred_mortgage_institution: String,
    pub preferred_buyers_lawyer: String,
    pub preferred_sellers_lawyer: String,
    pub preferred_surveyer: String,
    pub uploaded_legal_documents: Vec<String>,
}

impl Property {
    pub fn new(
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
    ) -> Self {
        Property {
            record_id,
            owners_full_name,
            owners_account_id: env::signer_account_id().to_string(),
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
            interested_customers: vec![],
            prepare: false,
            offer: false,
            due_dilligence: false,
            exchange: false,
            validated: false,
            completion: false,
            other_property_attributes,
            approval_in_principles: vec![],
            real_estate_proposed_transactions: vec![],
            formal_offers: vec![],
            memorandum_of_sales_agreements: vec![],
            preferred_mortgage_institution: "N/A".to_string(),
            preferred_buyers_lawyer: "N/A".to_string(),
            preferred_sellers_lawyer: "N/A".to_string(),
            preferred_surveyer: "N/A".to_string(),
            uploaded_legal_documents: vec![],
        }
    }
}
