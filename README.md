Rusty Store Inventory Management System
📌 Project Overview

The Rusty Store Inventory Management System is a simple command-line application written in Rust.
It helps small retail stores manage their inventory, sales, and purchases in an easy and beginner-friendly way.

The system is entirely text-based and provides a menu-driven interface for store managers.

✨ Features
1. Inventory Management

Add new products to the inventory.

Edit product details (name, description, price, quantity).

Delete products.

View all available products.

2. Sales Management

Record sales transactions.

Update inventory automatically when items are sold.

Track total sales and profit made from each transaction.

Handle errors (e.g., selling more items than available).

3. Purchase Management

Record purchase transactions.

Update inventory when new stock is purchased.

Calculate the total purchase cost.

4. Reporting

Generate reports showing:

Current inventory list.

Sales history.

Purchase history.

5. Error Handling

Prevents invalid inputs.

Handles out-of-stock cases.

Ensures that edits/deletes only apply to existing items.

6. Security

Basic authentication (username + password) for store managers.

Prevents unauthorized access to inventory and transactions.

7. User Interface

Simple text-based menu.

Beginner-friendly commands.

🛠️ Tech Stack

Language: Rust

Data Structures:

Vec → To store lists of products, sales, and purchases.

HashMap → To map product names/IDs with their details for quick lookups.

🚀 How to Run

Clone this repository

git clone https://github.com/your-username/rusty-store.git
cd rusty-store


Build the project

cargo build


Run the project

cargo run

📖 Example Usage
Welcome to Rusty Store!
Please log in:
Username: admin
Password: *****

1. Manage Inventory
2. Record Sale
3. Record Purchase
4. Reports
5. Exit
Enter your choice:


Example workflow:

Add product → Shampoo, Price: 50, Qty: 20

Record sale → Shampoo, Qty: 5

Inventory updates → Shampoo, Qty: 15

🧑‍💻 Future Improvements

Save/load inventory data to a file or database.

Add multiple user roles (e.g., cashier, manager).

Support for discounts and tax calculation.

👨‍🏫 Author

Youssef Ahmed
