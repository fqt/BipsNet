// Importing tools from the near sdk library that we'll use
#[allow(unused_imports)]
use near_sdk::near_bindgen;
#[allow(unused_imports)]
use near_sdk::collections::UnorderedMap;
use near_sdk::env;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;	

use std::collections::HashMap;

#[cfg(test)]
mod apptests;

#[cfg(test)]
#[allow(unused_imports)]
use crate::apptests::{testproperty, testuser};


// adding user and property modules to the scope
mod user;
mod property;

// importing the user & property module
#[allow(unused_imports)]
use crate::user::user::User;

#[allow(unused_imports)]
use crate::property::property::Property;

pub type AccountId = String;

#[near_bindgen]
#[allow(dead_code)]
pub struct Contract {
	owner: AccountId,
	users: HashMap<String, User>,
	properties: HashMap<String, Property>
}

#[near_bindgen]
impl Contract {
	pub fn new(owner: AccountId) -> Self{
		let users: HashMap<String, User> = HashMap::new();
		let properties: HashMap<String, Property> = HashMap::new();

		Contract {
			owner,
			users,
			properties,
		}
	}

	
	pub fn register_new_portal_user(&mut self, full_name: String, usertype: String, organization: String){
		// for now we assume that every user taking part in the system should at least have a near wallet id
		self.users.insert(env::signer_account_id().to_string(), User::new(full_name.to_string(), usertype.to_string(), organization.to_string()));
		env::log_str("user details updated Successfully");
	}

	// registration of the new property.
	pub fn register_new_property(&mut self, owners_full_name: String, property_address: String, property_description: String, improvements: String, asking_price_from_seller: u128, energy_certificate: String, floor_plan: String, gas_certificate: String, environmental_assesment: String, image1: String, image2: String, image3: String, image4: String, image5: String, image6: String){
		let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
		match &self.properties.get(&rand_string){
			Some(_) => {
				env::log_str("Hash Clash try again");
			},
			None => {
				self.properties.insert(rand_string.to_string(), Property::new(rand_string.to_string(), owners_full_name.to_string(), property_address.to_string(), property_description.to_string(), improvements.to_string(), asking_price_from_seller, energy_certificate.to_string(), floor_plan.to_string(), gas_certificate.to_string(), environmental_assesment.to_string(), image1.to_string(), image2.to_string(), image3.to_string(), image4.to_string(), image5.to_string(), image6.to_string()));
				env::log_str("Data Saved Successfully");
			}
		}		
	}

	pub fn get_all_properties(&self)->&HashMap<String, Property>{
		let properties = &self.properties;
		properties
	}

	pub fn count_properties(&self)->usize{
		let properties = &self.properties.keys().len();
		*properties
	}
}



