use near_sdk::{borsh::{self, BorshSerialize, BorshDeserialize}};
use near_sdk::env;
use near_sdk::serde::Serialize;
use near_sdk::serde::Deserialize;

pub type AccountId = String;

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
// files stored in arweave
pub struct Property {
	record_id: String,
	owners_full_name: String,
	agents_account_id: AccountId,
	property_address: String,
	property_description: String,
	improvements: String,
	asking_price_from_seller: u128,
	energy_certificate: String,
	floor_plan: String,
	gas_certificate: String,
	environmental_assesment: String,
	image1: String,
	image2: String,
	image3: String,
	image4: String,
	image5: String,
	image6: String,
	prepare: bool,
	offer: bool,
	due_dilligence: bool,
	exchange: bool,
	completion: bool,
}

impl Property {
	pub fn new(record_id: String, owners_full_name: String, property_address: String, property_description: String, improvements: String, asking_price_from_seller: u128, energy_certificate: String, floor_plan: String, gas_certificate: String, environmental_assesment: String, image1: String, image2: String, image3: String, image4: String, image5: String, image6: String)->Self{
		Property {
			record_id,
			agents_account_id: env::signer_account_id().to_string(),
			owners_full_name,
			property_address,
			property_description,
			improvements,
			asking_price_from_seller,
			energy_certificate,
			floor_plan,
			gas_certificate,
			environmental_assesment,
			image1,
			image2,
			image3,
			image4,
			image5,
			image6,
			prepare: false,
			offer: false,
			due_dilligence: false,
			exchange: false,
			completion: false,
		}
	}
}