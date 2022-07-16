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
pub fn test_approval_in_principle() {}
