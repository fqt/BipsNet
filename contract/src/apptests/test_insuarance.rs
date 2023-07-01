use crate::insurance_firm::insurance_firm::InsuranceContract;
use near_sdk::Balance;
use near_sdk::{env, AccountId};

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::env;

    #[test]
    fn test_create_property_policy() {

        let mut contract = InsuranceContract::default();

        let property_id = "property-1".to_string();
        let coverage_amount: Balance = 100;
        let property_address = "123 Main St".to_string();
        let start_date: u64 = 1674555600; // (UNIX timestamp)
        let end_date: u64 = 1674642000; //  (UNIX timestamp)
        let insured_items = vec!["item-1".to_string(), "item-2".to_string()];

        contract.create_property_policy(
            property_id.clone(),
            coverage_amount,
            property_address.clone(),
            start_date,
            end_date,
            insured_items.clone(),
        );

        let policy = contract.get_property_policy(property_id.clone()).unwrap();
        assert_eq!(policy.property_id, property_id);
        assert_eq!(policy.coverage_amount, coverage_amount);
        assert_eq!(policy.property_address, property_address);
        assert_eq!(policy.start_date, start_date);
        assert_eq!(policy.end_date, end_date);
        assert_eq!(policy.insured_items, insured_items);
    }

    #[test]
    fn test_update_property_policy() {
        let mut contract = InsuranceContract::default();

        let property_id = "property-1".to_string();
        let coverage_amount: Balance = 100;
        let property_address = "123 Main St".to_string();
        let start_date: u64 = 1674555600; // (UNIX timestamp)
        let end_date: u64 = 1674642000; // (UNIX timestamp)
        let insured_items = vec!["item-1".to_string(), "item-2".to_string()];

        contract.create_property_policy(
            property_id.clone(),
            coverage_amount,
            property_address.clone(),
            start_date,
            end_date,
            insured_items.clone(),
        );

        let new_coverage_amount: Balance = 200;
        let new_property_address = "456 Elm St".to_string();
        let new_start_date: u64 = 1674728400; // (UNIX timestamp)
        let new_end_date: u64 = 1674814800; // (UNIX timestamp)
        let new_insured_items = vec!["item-3".to_string(), "item-4".to_string()];

        contract.update_property_policy(
            property_id.clone(),
            new_coverage_amount,
            new_property_address.clone(),
            new_start_date,
            new_end_date,
            new_insured_items.clone(),
        );

        let policy = contract.get_property_policy(property_id.clone()).unwrap();
        assert_eq!(policy.property_id, property_id);
        assert_eq!(policy.coverage_amount, new_coverage_amount);
        assert_eq!(policy.property_address, new_property_address);
        assert_eq!(policy.start_date, new_start_date);
        assert_eq!(policy.end_date, new_end_date);
        assert_eq!(policy.insured_items, new_insured_items);
    }

    #[test]
    fn test_create_mortgage_policy() {
        let mut contract = InsuranceContract::default();

        let loan_id = "loan-1".to_string();
        let borrower = env::predecessor_account_id();
        let coverage_amount: Balance = 1000;
        let property_address = "123 Main St".to_string();
        let start_date: u64 = 1674555600; // (UNIX timestamp)
        let end_date: u64 = 1674642000; // (UNIX timestamp)
        let premium: Balance = 500;
        let interest_rate: f64 = 0.05;

        contract.create_mortgage_policy(
            loan_id.clone(),
            borrower.clone(),
            coverage_amount,
            property_address.clone(),
            start_date,
            end_date,
            premium,
            interest_rate,
        );

        let policy = contract.get_mortgage_policy(loan_id.clone()).unwrap();
        assert_eq!(policy.loan_id, loan_id);
        assert_eq!(policy.borrower, borrower);
        assert_eq!(policy.coverage_amount, coverage_amount);
        assert_eq!(policy.property_address, property_address);
        assert_eq!(policy.start_date, start_date);
        assert_eq!(policy.end_date, end_date);
        assert_eq!(policy.premium, premium);
        assert_eq!(policy.interest_rate, interest_rate);
    }

    #[test]
    fn test_update_mortgage_policy() {
        let mut contract = InsuranceContract::default();

        let loan_id = "loan-1".to_string();
        let borrower = env::predecessor_account_id();
        let coverage_amount: Balance = 1000;
        let property_address = "123 Main St".to_string();
        let start_date: u64 = 1674555600; // (UNIX timestamp)
        let end_date: u64 = 1674642000; // (UNIX timestamp)
        let premium: Balance = 500;
        let interest_rate: f64 = 0.05;

        contract.create_mortgage_policy(
            loan_id.clone(),
            borrower.clone(),
            coverage_amount,
            property_address.clone(),
            start_date,
            end_date,
            premium,
            interest_rate,
        );

        let new_borrower = env::predecessor_account_id();
        let new_coverage_amount: Balance = 2000;
        let new_property_address = "456 Elm St".to_string();
        let new_start_date: u64 = 1674728400; //  (UNIX timestamp)
        let new_end_date: u64 = 1674814800; // (UNIX timestamp)
        let new_premium: Balance = 1000;
        let new_interest_rate: f64 = 0.06;

        contract.update_mortgage_policy(
            loan_id.clone(),
            new_borrower.clone(),
            new_coverage_amount,
            new_property_address.clone(),
            new_start_date,
            new_end_date,
            new_premium,
            new_interest_rate,
        );

        let policy = contract.get_mortgage_policy(loan_id.clone()).unwrap();
        assert_eq!(policy.loan_id, loan_id);
        assert_eq!(policy.borrower, new_borrower);
        assert_eq!(policy.coverage_amount, new_coverage_amount);
        assert_eq!(policy.property_address, new_property_address);
        assert_eq!(policy.start_date, new_start_date);
        assert_eq!(policy.end_date, new_end_date);
        assert_eq!(policy.premium, new_premium);
        assert_eq!(policy.interest_rate, new_interest_rate);
    }


    #[test]
    fn test_create_liability_policy() {
        let mut contract = InsuranceContract::default();

        let insured_account_id = env::predecessor_account_id();
        let coverage_amount: Balance = 1000;
        let policy_holder = env::predecessor_account_id();
        let start_date: u64 = 1674555600; // (UNIX timestamp)
        let end_date: u64 = 1674642000; // (UNIX timestamp)
        let premium: Balance = 500;
        let coverage_type = "general".to_string();

        contract.create_liability_policy(
            insured_account_id.clone(),
            coverage_amount,
            policy_holder.clone(),
            start_date,
            end_date,
            premium,
            coverage_type.clone(),
        );

        let policy = contract.get_liability_policy(insured_account_id.clone()).unwrap();
        assert_eq!(policy.insured_account_id, insured_account_id);
        assert_eq!(policy.coverage_amount, coverage_amount);
        assert_eq!(policy.policy_holder, policy_holder);
        assert_eq!(policy.start_date, start_date);
        assert_eq!(policy.end_date, end_date);
        assert_eq!(policy.premium, premium);
        assert_eq!(policy.coverage_type, coverage_type);
    }

    #[test]
    fn test_update_liability_policy() {
        let mut contract = InsuranceContract::default();

        let insured_account_id = env::predecessor_account_id();
        let coverage_amount: Balance = 1000;
        let policy_holder = env::predecessor_account_id();
        let start_date: u64 = 1674555600; // (UNIX timestamp)
        let end_date: u64 = 1674642000; // (UNIX timestamp)
        let premium: Balance = 500;
        let coverage_type = "general".to_string();

        contract.create_liability_policy(
            insured_account_id.clone(),
            coverage_amount,
            policy_holder.clone(),
            start_date,
            end_date,
            premium,
            coverage_type.clone(),
        );

        let new_coverage_amount: Balance = 2000;
        let new_policy_holder = env::predecessor_account_id();
        let new_start_date: u64 = 1674728400; //(UNIX timestamp)
        let new_end_date: u64 = 1674814800; // (UNIX timestamp)
        let new_premium: Balance = 1000;
        let new_coverage_type = "specific".to_string();

        contract.update_liability_policy(
            insured_account_id.clone(),
            new_coverage_amount,
            new_policy_holder.clone(),
            new_start_date,
            new_end_date,
            new_premium,
            new_coverage_type.clone(),
        );

        let policy = contract.get_liability_policy(insured_account_id.clone()).unwrap();
        assert_eq!(policy.insured_account_id, insured_account_id);
        assert_eq!(policy.coverage_amount, new_coverage_amount);
        assert_eq!(policy.policy_holder, new_policy_holder);
        assert_eq!(policy.start_date, new_start_date);
        assert_eq!(policy.end_date, new_end_date);
        assert_eq!(policy.premium, new_premium);
        assert_eq!(policy.coverage_type, new_coverage_type);
    }

    #[test]
    fn test_create_title_policy() {
        let mut contract = InsuranceContract::default();

        let property_id = "property-1".to_string();
        let policy_holder = env::predecessor_account_id();
        let coverage_amount: Balance = 1000;
        let start_date: u64 = 1674555600; //(UNIX timestamp)
        let end_date: u64 = 1674642000; // (UNIX timestamp)
        let title_company = "ABC Title Company".to_string();

        contract.create_title_policy(
            property_id.clone(),
            policy_holder.clone(),
            coverage_amount,
            start_date,
            end_date,
            title_company.clone(),
        );

        let policy = contract.get_title_policy(property_id.clone()).unwrap();
        assert_eq!(policy.property_id, property_id);
        assert_eq!(policy.policy_holder, policy_holder);
        assert_eq!(policy.coverage_amount, coverage_amount);
        assert_eq!(policy.start_date, start_date);
        assert_eq!(policy.end_date, end_date);
        assert_eq!(policy.title_company, title_company);
    }

    #[test]
    fn test_update_title_policy() {
        let mut contract = InsuranceContract::default();

        let property_id = "property-1".to_string();
        let policy_holder = env::predecessor_account_id();
        let coverage_amount: Balance = 1000;
        let start_date: u64 = 1674555600; //  (UNIX timestamp)
        let end_date: u64 = 1674642000; //(UNIX timestamp)
        let title_company = "ABC Title Company".to_string();

        contract.create_title_policy(
            property_id.clone(),
            policy_holder.clone(),
            coverage_amount,
            start_date,
            end_date,
            title_company.clone(),
        );

        let new_policy_holder = env::predecessor_account_id();
        let new_coverage_amount: Balance = 2000;
        let new_start_date: u64 = 1674728400; // (UNIX timestamp)
        let new_end_date: u64 = 1674814800; // (UNIX timestamp)
        let new_title_company = "XYZ Title Company".to_string();

        contract.update_title_policy(
            property_id.clone(),
            new_policy_holder.clone(),
            new_coverage_amount,
            new_start_date,
            new_end_date,
            new_title_company.clone(),
        );

        let policy = contract.get_title_policy(property_id.clone()).unwrap();
        assert_eq!(policy.property_id, property_id);
        assert_eq!(policy.policy_holder, new_policy_holder);
        assert_eq!(policy.coverage_amount, new_coverage_amount);
        assert_eq!(policy.start_date, new_start_date);
        assert_eq!(policy.end_date, new_end_date);
        assert_eq!(policy.title_company, new_title_company);
    }

    #[test]
    fn test_create_investment_policy() {
        let mut contract = InsuranceContract::default();

        let property_id = "property-1".to_string();
        let investor = env::predecessor_account_id();
        let investment_amount: Balance = 5000;
        let start_date: u64 = 1674555600; // (UNIX timestamp)
        let end_date: u64 = 1674642000; // (UNIX timestamp)
        let investment_type = "real estate".to_string();

        contract.create_investment_policy(
            property_id.clone(),
            investor.clone(),
            investment_amount,
            start_date,
            end_date,
            investment_type.clone(),
        );

        let policy = contract.get_investment_policy(property_id.clone()).unwrap();
        assert_eq!(policy.property_id, property_id);
        assert_eq!(policy.investor, investor);
        assert_eq!(policy.investment_amount, investment_amount);
        assert_eq!(policy.start_date, start_date);
        assert_eq!(policy.end_date, end_date);
        assert_eq!(policy.investment_type, investment_type);
    }

    #[test]
    fn test_update_investment_policy() {
        let mut contract = InsuranceContract::default();

        let property_id = "property-1".to_string();
        let investor = env::predecessor_account_id();
        let investment_amount: Balance = 5000;
        let start_date: u64 = 1674555600; //  (UNIX timestamp)
        let end_date: u64 = 1674642000; // (UNIX timestamp)
        let investment_type = "real estate".to_string();

        contract.create_investment_policy(
            property_id.clone(),
            investor.clone(),
            investment_amount,
            start_date,
            end_date,
            investment_type.clone(),
        );

        let new_investor = env::predecessor_account_id();
        let new_investment_amount: Balance = 10000;
        let new_start_date: u64 = 1674728400; //  (UNIX timestamp)
        let new_end_date: u64 = 1674814800; // (UNIX timestamp)
        let new_investment_type = "stocks".to_string();

        contract.update_investment_policy(
            property_id.clone(),
            new_investor.clone(),
            new_investment_amount,
            new_start_date,
            new_end_date,
            new_investment_type.clone(),
        );

        let policy = contract.get_investment_policy(property_id.clone()).unwrap();
        assert_eq!(policy.property_id, property_id);
        assert_eq!(policy.investor, new_investor);
        assert_eq!(policy.investment_amount, new_investment_amount);
        assert_eq!(policy.start_date, new_start_date);
        assert_eq!(policy.end_date, new_end_date);
        assert_eq!(policy.investment_type, new_investment_type);
    }
}
