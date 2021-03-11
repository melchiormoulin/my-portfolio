mod transaction;
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use yahoo_finance_api as yahoo;
use clap::{App, Arg};
mod json_file_parser;
use tokio::runtime::Runtime;


async fn get_quote(ticker:&str) -> Result<f64, yahoo::YahooError> {
    let provider = yahoo::YahooConnector::new();

    // get the latest quotes in 1 minute intervals
    let response = provider.get_latest_quotes(ticker, "1m").await.unwrap();
    // extract just the latest valid quote summery
    let quote = response.last_quote()?;
    Ok(quote.close)
}

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
    println!("{}", serde_json::to_string(&wallet).unwrap());   
    let rt  = Runtime::new().unwrap();
    for ticker in wallet.tickers {
        let quotes = rt.block_on(get_quote(&ticker[..]));
        println!("{} quotes of the last minute: {:?}", ticker,quotes.unwrap());
    }


}
