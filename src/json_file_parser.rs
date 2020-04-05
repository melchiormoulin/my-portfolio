use crate::transaction;
use std::fs;
pub fn get_transactions(filename: &str) -> transaction::Transactions {
    let json_data = fs::read_to_string(filename).expect("Error reading the file.");
    let transaction_tab = serde_json::from_str(&json_data).expect("Error parsing json file.");
    transaction::Transactions {
        transactions: transaction_tab,
    }
}
