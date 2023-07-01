use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId, Balance,Promise,log, near_bindgen};
use near_sdk::serde::Serialize;
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PropertyPolicy {
    pub property_id: String,
    pub coverage_amount: Balance,
    pub property_address: String,
    pub start_date: u64,
    pub end_date: u64,
    pub insured_items: Vec<String>,
    
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MortgagePolicy {
    pub loan_id: String,
    pub borrower: AccountId,
    pub coverage_amount: Balance,
    pub property_address: String,
    pub start_date: u64,
    pub end_date: u64,
    pub premium: Balance,
    pub interest_rate: f64,

}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LiabilityPolicy {
    pub insured_account_id: AccountId,
    pub coverage_amount: Balance,
    pub policy_holder: AccountId,
    pub start_date: u64,
    pub end_date: u64,
    pub premium: Balance,
    pub coverage_type: String,
  
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TitlePolicy {
    pub property_id: String,
    pub policy_holder: AccountId,
    pub coverage_amount: Balance,
    pub start_date: u64,
    pub end_date: u64,
    pub title_company: String,
  
}
#[derive(BorshSerialize, BorshDeserialize)]
pub struct InvestmentPolicy {
    pub property_id: String,
    pub investor: AccountId,
    pub investment_amount: Balance,
    pub start_date: u64,
    pub end_date: u64,
    pub investment_type: String,
   
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct InsuranceContract {
    property_policies: HashMap<String, PropertyPolicy>,
    mortgage_policies: HashMap<String, MortgagePolicy>,
    liability_policies: HashMap<AccountId, LiabilityPolicy>,
    title_policies: HashMap<String, TitlePolicy>,
    investment_policies: HashMap<String, InvestmentPolicy>,
}

impl Default for InsuranceContract {
    fn default() -> Self {
        Self {
            property_policies: HashMap::new(),
            mortgage_policies: HashMap::new(),
            liability_policies: HashMap::new(),
            title_policies: HashMap::new(),
            investment_policies:HashMap::new(),
        }
    }
}

#[near_bindgen]
impl InsuranceContract {
    
    pub fn create_property_policy(
        &mut self,
        property_id: String,
        coverage_amount: Balance,
        property_address: String,
        start_date: u64,
        end_date: u64,
        insured_items: Vec<String>,
        
    ) {
        let property_id_copy = property_id.clone();
        let policy = PropertyPolicy {
            property_id,
            coverage_amount,
            property_address,
            start_date,
            end_date,
            insured_items,
           
        };
        self.property_policies.insert(property_id_copy, policy);
    }

    pub fn get_property_policy(&self, property_id: String) -> Option<&PropertyPolicy> {
        self.property_policies.get(&property_id)
    }

    pub fn update_property_policy(
        &mut self,
        property_id: String,
        coverage_amount: Balance,
        property_address: String,
        start_date: u64,
        end_date: u64,
        insured_items: Vec<String>,
       
    ) {
        if let Some(mut policy) = self.property_policies.get_mut(&property_id) {
            policy.coverage_amount = coverage_amount;
            policy.property_address = property_address;
            policy.start_date = start_date;
            policy.end_date = end_date;
            policy.insured_items = insured_items;
            // self.property_policies.insert(property_id.clone(), &policy.clone());
        }
    }

    
    pub fn create_mortgage_policy(
        &mut self,
        loan_id: String,
        borrower: AccountId,
        coverage_amount: Balance,
        property_address: String,
        start_date: u64,
        end_date: u64,
        premium: Balance,
        interest_rate: f64,
        
    ) {

        let loan_id_copy = loan_id.clone();

        let policy = MortgagePolicy {
            loan_id,
            borrower,
            coverage_amount,
            property_address,
            start_date,
            end_date,
            premium,
            interest_rate,
           
        };
        self.mortgage_policies.insert(loan_id_copy, policy);
    }

    pub fn get_mortgage_policy(&self, loan_id: String) -> Option<&MortgagePolicy> {
        self.mortgage_policies.get(&loan_id)
    }

    pub fn update_mortgage_policy(
        &mut self,
        loan_id: String,
        borrower: AccountId,
        coverage_amount: Balance,
        property_address: String,
        start_date: u64,
        end_date: u64,
        premium: Balance,
        interest_rate: f64,
        
    ) {
        if let Some(mut policy) = self.mortgage_policies.get_mut(&loan_id) {
            policy.borrower = borrower;
            policy.coverage_amount = coverage_amount;
            policy.property_address = property_address;
            policy.start_date = start_date;
            policy.end_date = end_date;
            policy.premium = premium;
            policy.interest_rate = interest_rate;
          
            // self.mortgage_policies.insert(loan_id, policy);
        }
    }

    // Functions for liability insurance policies
    pub fn create_liability_policy(
        &mut self,
        insured_account_id: AccountId,
        coverage_amount: Balance,
        policy_holder: AccountId,
        start_date: u64,
        end_date: u64,
        premium: Balance,
        coverage_type: String,
        
    ) {

        let insured_account_id_copy = insured_account_id.clone();

        let policy = LiabilityPolicy {
            insured_account_id,
            coverage_amount,
            policy_holder,
            start_date,
            end_date,
            premium,
            coverage_type,
            
        };
        self.liability_policies.insert(insured_account_id_copy, policy);
    }

    pub fn get_liability_policy(&self, insured_account_id: AccountId) -> Option<&LiabilityPolicy> {
        self.liability_policies.get(&insured_account_id)
    }

    pub fn update_liability_policy(
        &mut self,
        insured_account_id: AccountId,
        coverage_amount: Balance,
        policy_holder: AccountId,
        start_date: u64,
        end_date: u64,
        premium: Balance,
        coverage_type: String,
       
    ) {
        if let Some(mut policy) = self.liability_policies.get_mut(&insured_account_id) {
            policy.coverage_amount = coverage_amount;
            policy.policy_holder = policy_holder;
            policy.start_date = start_date;
            policy.end_date = end_date;
            policy.premium = premium;
            policy.coverage_type = coverage_type;
            
            // self.liability_policies.insert(insured_account_id, policy);
        }
    }

    // Functions for title insurance policies
    pub fn create_title_policy(
        &mut self,
        property_id: String,
        policy_holder: AccountId,
        coverage_amount: Balance,
        start_date: u64,
        end_date: u64,
        title_company: String,
        
    ) {


        let property_id_copy  = property_id.clone();

        let policy = TitlePolicy {
            property_id,
            policy_holder,
            coverage_amount,
            start_date,
            end_date,
            title_company,
            
        };
        self.title_policies.insert(property_id_copy, policy);
    }

    pub fn get_title_policy(&self, property_id: String) -> Option<&TitlePolicy> {
        self.title_policies.get(&property_id)
    }

    pub fn update_title_policy(
        &mut self,
        property_id: String,
        policy_holder: AccountId,
        coverage_amount: Balance,
        start_date: u64,
        end_date: u64,
        title_company: String,
        
    ) {
        if let Some(policy) = self.title_policies.get_mut(&property_id) {
            policy.policy_holder = policy_holder;
            policy.coverage_amount = coverage_amount;
            policy.start_date = start_date;
            policy.end_date = end_date;
            policy.title_company = title_company;


            // self.title_policies.insert(property_id.clone(), policy);
        }
    }
    pub fn create_investment_policy(
        &mut self,
        property_id: String,
        investor: AccountId,
        investment_amount: Balance,
        start_date: u64,
        end_date: u64,
        investment_type: String,
       
    ) {


        let property_id_copy = property_id.clone();

        let policy = InvestmentPolicy {
            property_id,
            investor,
            investment_amount,
            start_date,
            end_date,
            investment_type,
            
        };
        self.investment_policies.insert(property_id_copy, policy);
    }

    pub fn get_investment_policy(&self, property_id: String) -> Option<&InvestmentPolicy> {
        self.investment_policies.get(&property_id)
    }

    pub fn update_investment_policy(
        &mut self,
        property_id: String,
        investor: AccountId,
        investment_amount: Balance,
        start_date: u64,
        end_date: u64,
        investment_type: String,
        
    ) {
        if let Some(mut policy) = self.investment_policies.get_mut(&property_id) {
            policy.investor = investor;
            policy.investment_amount = investment_amount;
            policy.start_date = start_date;
            policy.end_date = end_date;
            policy.investment_type = investment_type;
            
            // self.investment_policies.insert(property_id, policy);
        }
    }

   
}