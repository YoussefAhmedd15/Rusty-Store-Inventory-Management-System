use std::io;
use rusty_store::store::Store;

fn main() {
    let mut store = Store::new("1234");

    if !store.authenticate() {
        println!("Authentication failed. Exiting...");
        return;
    }

    loop {
        println!("\n--- Rusty Store Menu ---");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Record Sale");
        println!("5. Record Purchase");
        println!("6. Generate Reports");
        println!("7. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => store.add_product(),
            "2" => store.edit_product(),
            "3" => store.delete_product(),
            "4" => store.record_sale(),
            "5" => store.record_purchase(),
            "6" => store.generate_reports(),
            "7" => {
                println!("Saving data before exit...");
                if let Err(e) = store.save_data() {
                    println!("Error saving data: {}", e);
                }
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}
