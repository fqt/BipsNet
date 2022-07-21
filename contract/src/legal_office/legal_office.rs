// sellers lawyer
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DraftOfSalesAgreement {
    record_id_for_property: String,
    sellers_name: String,
    buyers_name: String,
    property_address: String,
    title_id_from_the_registry: String, // digital token id
    purchase_price: u128,
    specific_terms_and_condition: String, // specific terms and conditions of the deal if any
    date_of_completion: String,
    standard_conditions_of_sales: String,
}

impl DraftOfSalesAgreement {
    pub fn new(
        record_id_for_property: String,
        sellers_name: String,
        buyers_name: String,
        property_address: String,
        title_id_from_the_registry: String,
        purchase_price: u128,
        specific_terms_and_condition: String,
        date_of_completion: String,
        standard_conditions_of_sales: String,
    ) -> Self {
        DraftOfSalesAgreement {
            record_id_for_property,
            sellers_name,
            buyers_name,
            property_address,
            title_id_from_the_registry,
            purchase_price,
            specific_terms_and_condition,
            date_of_completion,
            standard_conditions_of_sales,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn create_draft_of_sales_agreement(
        &mut self,
        record_id_for_property: String,
        sellers_name: String,
        buyers_name: String,
        property_address: String,
        title_id_from_the_registry: String,
        purchase_price: String,
        specific_terms_and_condition: String,
        date_of_completion: String,
        standard_conditions_of_sales: String,
    ) {
        // happens after the property has been tokenized
    }
}
