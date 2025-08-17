use serde::{Deserialize, Serialize};

// Purchase record
#[derive(Serialize, Deserialize)]
pub struct Purchase {
    pub product_name: String,
    pub quantity: u32,
    pub purchase_price: f64,
    pub total_cost: f64,
}

impl Purchase {
    pub fn new(product_name: String, quantity: u32, purchase_price: f64, total_cost: f64) -> Self {
        Purchase {
            product_name,
            quantity,
            purchase_price,
            total_cost,
        }
    }
}
