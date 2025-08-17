use serde::{Deserialize, Serialize};

// Product structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}

impl Product {
    pub fn new(name: String, description: String, price: f64, quantity: u32) -> Self {
        Product {
            name,
            description,
            price,
            quantity,
        }
    }
}
