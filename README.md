🦀 Rusty Store Inventory Management System

A simple Inventory Management System written in Rust for a small retail store.
This project helps store managers manage products, track sales, record purchases, and generate reports — all from a clean text-based interface.

✨ Features

📦 Inventory Management

Add, edit, and delete products

Track product name, description, price, and quantity

💰 Sales Management

Record sales transactions (product, quantity, price)

Automatically calculate total sales & profit

🛒 Purchase Management

Record purchase transactions (product, quantity, cost)

Calculate total purchase cost

📊 Reports

View inventory list

View sales history

View purchase history

🔐 Security

Basic authentication for store managers

Prevents unauthorized access

⚡ Error Handling

Handles invalid inputs

Prevents selling out-of-stock items

🖥️ User Interface

Easy text-based menu system

Beginner-friendly navigation

🛠️ Built With

Rust 🦀

Rust Standard Library (beginner-friendly, minimal dependencies)

🚀 Getting Started
1. Clone the repository
git clone https://github.com/your-username/rusty-store-inventory.git
cd rusty-store-inventory

2. Run the project
cargo run

🧪 Testing

Run the test suite:

cargo test

📂 Project Structure
src/
 ├── main.rs         # Entry point with menu and navigation
 ├── inventory.rs    # Inventory management logic
 ├── sales.rs        # Sales management logic
 ├── purchases.rs    # Purchases management logic
 ├── reports.rs      # Report generation
 └── auth.rs         # Authentication & access control

🎯 Learning Goals

This project demonstrates Rust concepts such as:

Ownership & Borrowing

Structs & Enums

Traits & Implementations

Error Handling (Result & Option)

Iterators & Collections (Vec, HashMap)

📜 License

This project is open source and available under the MIT License.
