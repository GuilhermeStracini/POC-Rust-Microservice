use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
}

impl Item {
    pub fn new(id: u32, name: &str, description: &str) -> Self {
        Item {
            id,
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

// This module defines the Item struct and its associated methods.
// We will use this struct to represent our core data structure in the microservice.
