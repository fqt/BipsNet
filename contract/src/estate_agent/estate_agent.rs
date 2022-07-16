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
    sellers_lawyer: String,
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
