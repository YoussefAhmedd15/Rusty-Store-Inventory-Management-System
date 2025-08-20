use std::collections::HashMap;
use std::io;
use std::fs;
use serde_json;
use crate::product::Product;
use crate::sale::Sale;
use crate::purchase::Purchase;

pub struct Store {
    pub inventory: HashMap<String, Product>,
    pub sales: Vec<Sale>,
    pub purchases: Vec<Purchase>,
    pub manager_password: String,
}

impl Store {
    pub fn new(password: &str) -> Store {
        let mut store = Store {
            inventory: HashMap::new(),
            sales: Vec::new(),
            purchases: Vec::new(),
            manager_password: password.to_string(),
        };
        
        
        store.load_data();
        store
    }

    
    pub fn save_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        let inventory_data = serde_json::to_string_pretty(&self.inventory)?;
        fs::write("inventory.json", inventory_data)?;
        
        
        let sales_data = serde_json::to_string_pretty(&self.sales)?;
        fs::write("sales.json", sales_data)?;
        
        
        let purchases_data = serde_json::to_string_pretty(&self.purchases)?;
        fs::write("purchases.json", purchases_data)?;
        
        println!("Data saved successfully!");
        Ok(())
    }

    
    pub fn load_data(&mut self) {
        
        if let Ok(data) = fs::read_to_string("inventory.json") {
            if let Ok(inventory) = serde_json::from_str::<HashMap<String, Product>>(&data) {
                self.inventory = inventory;
                println!("Inventory loaded successfully!");
            }
        }
        
        
        if let Ok(data) = fs::read_to_string("sales.json") {
            if let Ok(sales) = serde_json::from_str::<Vec<Sale>>(&data) {
                self.sales = sales;
                println!("Sales history loaded successfully!");
            }
        }
        
        
        if let Ok(data) = fs::read_to_string("purchases.json") {
            if let Ok(purchases) = serde_json::from_str::<Vec<Purchase>>(&data) {
                self.purchases = purchases;
                println!("Purchase history loaded successfully!");
            }
        }
    }

    pub fn add_product(&mut self) {
        let mut input = String::new();

        println!("Enter product name:");
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();
        input.clear();

        println!("Enter description:");
        io::stdin().read_line(&mut input).unwrap();
        let description = input.trim().to_string();
        input.clear();

        println!("Enter price:");
        io::stdin().read_line(&mut input).unwrap();
        let price: f64 = input.trim().parse().unwrap_or(0.0);
        input.clear();

        println!("Enter quantity:");
        io::stdin().read_line(&mut input).unwrap();
        let quantity: u32 = input.trim().parse().unwrap_or(0);

        let product = Product::new(name.clone(), description, price, quantity);

        self.inventory.insert(name.clone(), product);
        println!("Product '{}' added successfully.", name);
        
        
        if let Err(e) = self.save_data() {
            println!("Warning: Could not save data: {}", e);
        }
    }

    pub fn edit_product(&mut self) {
        let mut input = String::new();
        println!("Enter product name to edit:");
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();

        if let Some(product) = self.inventory.get_mut(&name) {
            input.clear();
            println!("Enter new price:");
            io::stdin().read_line(&mut input).unwrap();
            product.price = input.trim().parse().unwrap_or(product.price);

            input.clear();
            println!("Enter new quantity:");
            io::stdin().read_line(&mut input).unwrap();
            product.quantity = input.trim().parse().unwrap_or(product.quantity);

            println!("Product '{}' updated.", name);
            
            
            if let Err(e) = self.save_data() {
                println!("Warning: Could not save data: {}", e);
            }
        } else {
            println!("Product not found.");
        }
    }

    pub fn delete_product(&mut self) {
        let mut input = String::new();
        println!("Enter product name to delete:");
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();

        if self.inventory.remove(&name).is_some() {
            println!("Product '{}' deleted.", name);
            
            
            if let Err(e) = self.save_data() {
                println!("Warning: Could not save data: {}", e);
            }
        } else {
            println!("Product not found.");
        }
    }

    pub fn record_sale(&mut self) {
        let mut input = String::new();
        println!("Enter product name sold:");
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();

        if let Some(product) = self.inventory.get_mut(&name) {
            input.clear();
            println!("Enter quantity sold:");
            io::stdin().read_line(&mut input).unwrap();
            let quantity: u32 = input.trim().parse().unwrap_or(0);

            if product.quantity >= quantity {
                product.quantity -= quantity;

                let sale_price = product.price;
                let total = sale_price * quantity as f64;
                let profit = total * 0.3;

                let sale = Sale::new(name.clone(), quantity, sale_price, total, profit);

                self.sales.push(sale);
                println!("Sale recorded. Total = {:.2}, Profit = {:.2}", total, profit);
                
                
                if let Err(e) = self.save_data() {
                    println!("Warning: Could not save data: {}", e);
                }
            } else {
                println!("Not enough stock available.");
            }
        } else {
            println!("Product not found.");
        }
    }

    pub fn record_purchase(&mut self) {
        let mut input = String::new();
        println!("Enter product name purchased:");
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();

        input.clear();
        println!("Enter quantity purchased:");
        io::stdin().read_line(&mut input).unwrap();
        let quantity: u32 = input.trim().parse().unwrap_or(0);

        input.clear();
        println!("Enter purchase price per item:");
        io::stdin().read_line(&mut input).unwrap();
        let purchase_price: f64 = input.trim().parse().unwrap_or(0.0);

        let total_cost = purchase_price * quantity as f64;

        if let Some(product) = self.inventory.get_mut(&name) {
            product.quantity += quantity;
        } else {
            let product = Product::new(name.clone(), "No description".to_string(), purchase_price, quantity);
            self.inventory.insert(name.clone(), product);
        }

        let purchase = Purchase::new(name.clone(), quantity, purchase_price, total_cost);

        self.purchases.push(purchase);
        println!("Purchase recorded. Total cost = {:.2}", total_cost);
        
        
        if let Err(e) = self.save_data() {
            println!("Warning: Could not save data: {}", e);
        }
    }

    pub fn generate_reports(&self) {
        println!("\n--- Inventory Report ---");
        for product in self.inventory.values() {
            println!(
                "Name: {}, Desc: {}, Price: {:.2}, Qty: {}",
                product.name, product.description, product.price, product.quantity
            );
        }

        println!("\n--- Sales Report ---");
        for sale in &self.sales {
            println!(
                "Product: {}, Qty: {}, Total: {:.2}, Profit: {:.2}",
                sale.product_name, sale.quantity, sale.total, sale.profit
            );
        }

        println!("\n--- Purchase Report --");
        for purchase in &self.purchases {
            println!(
                "Product: {}, Qty: {}, Total Cost: {:.2}",
                purchase.product_name, purchase.quantity, purchase.total_cost
            );
        }
    }

    pub fn authenticate(&self) -> bool {
        let mut input = String::new();
        println!("Enter manager password:");
        io::stdin().read_line(&mut input).unwrap();
        let entered = input.trim();
        entered == self.manager_password
    }
}
