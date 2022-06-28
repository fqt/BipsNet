// Importing tools from the near sdk library that we'll use
#[allow(unused_imports)]
use near_sdk::near_bindgen;
use near_sdk::collections::UnorderedMap;

#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(test)]
mod apptests;


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
	users: UnorderedMap<String, User>,
	properties: UnorderedMap<String, Property>
}

#[near_bindgen]
impl Contract {
	
}



