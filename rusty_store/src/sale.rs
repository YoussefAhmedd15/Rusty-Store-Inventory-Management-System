use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sale {
    pub product_name: String,
    pub quantity: u32,
    pub sale_price: f64,
    pub total: f64,
    pub profit: f64,
}

impl Sale {
    pub fn new(product_name: String, quantity: u32, sale_price: f64, total: f64, profit: f64) -> Self {
        Sale {
            product_name,
            quantity,
            sale_price,
            total,
            profit,
        }
    }
}
