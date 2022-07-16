use crate::Contract;
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, AccountId};

fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.signer_account_id(predecessor);
    builder.account_balance(1_000_000_000_000_000_000_000_000_000_000);
    builder
}

#[test]
pub fn new_property() {
    let felix = AccountId::new_unchecked("felix.testnet".to_string());
    let context = get_context(felix.clone());
    testing_env!(context.build());

    let mut contract = Contract::new(felix.to_string());
    contract.register_new_property(
        "Felix Awere".to_string(),
        "Kisumu".to_string(),
        "lorem ipsum dolor sit amet consectetur, ivano".to_string(),
        "Plenning to recolor the sailing".to_string(),
        1000,
        "path_to_energy_certificate".to_string(),
        "floor_plan".to_string(),
        "path_to_gas_certificate".to_string(),
        "path_to_environmental_assesment".to_string(),
        "image1".to_string(),
        "image2".to_string(),
        "image3".to_string(),
        "image4".to_string(),
        "image5".to_string(),
        "image6".to_string(),
    );
    println!("{:?}", contract.properties);
    assert_eq!(contract.count_properties(), 1);
}
