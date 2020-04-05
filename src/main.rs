mod transaction;
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
mod json_file_parser;
fn main() {
    let matches = App::new("My Portofolio Program")
        .version("0.1.0")
        .author("Melchior MOULIN")
        .about("Portfolio")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("transaction json file"),
        )
        .get_matches();
    let filename = matches.value_of("file").unwrap_or("./examples/wallet.json");
    let transactions = json_file_parser::get_transactions(filename);
    let wallet = transaction::Wallet::new(transactions);
    println!("{}",serde_json::to_string(&wallet).unwrap());
}
