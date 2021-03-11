extern crate chrono;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

use chrono::{DateTime, Utc};
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    source: Entity,
    destination: Entity,
    transaction_type: TransactionType,
    ticker: Ticker,
    asset: Asset,
    asset_quantity: f64,
    currency: Asset,
    currency_quantity: f64,
    currency_fees: Asset,
    currency_fees_quantity: f64,
    sent_date: DateTime<Utc>,
    received_date: DateTime<Utc>,
}

#[derive(Serialize, Copy, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    BUY,
    SELL,
    TRANSFER,
}

pub type Asset = String;

pub type Ticker = String;

pub type Entity = String;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    quantity_by_transaction_type_by_currency: HashMap<String, HashMap<TransactionType, f64>>,
    total_cost_by_transaction_type_by_currency: HashMap<String, HashMap<TransactionType, f64>>,
    current_quantity_by_currency: HashMap<String,f64>,
    pub assets : Vec<String>,
    pub tickers: HashSet<Ticker>
}
impl Wallet {
    pub fn new(transactions: Transactions) -> Wallet {
        Wallet {
            quantity_by_transaction_type_by_currency: transactions.get_quantity_by_transaction_type_by_asset(),
            total_cost_by_transaction_type_by_currency: transactions.get_total_cost_by_transaction_type_by_currency(),
            current_quantity_by_currency: transactions.get_current_quantity_by_asset(),
            assets: transactions.get_assets(),
            tickers : transactions.get_tickers()
        }
    }
}
impl Transactions {
    pub fn get_quantity_by_transaction_type_by_asset(
        &self,
    ) -> HashMap<String, HashMap<TransactionType, f64>> {
        self.transactions
            .clone()
            .into_iter()
            .group_by(|transaction| transaction.asset.clone())
            .into_iter()
            .map(|(asset, transaction)| {
                (
                    asset,
                    transaction
                        .into_iter()
                        .group_by(|transaction| transaction.transaction_type.clone())
                        .into_iter()
                        .map(|(transactiontype, transaction1)| {
                            (
                                transactiontype,
                                transaction1.map(|transac| transac.asset_quantity).sum(),
                            )
                        })
                        .collect(),
                )
            })
            .collect()
    }
    pub fn get_current_quantity_by_asset(
        &self,
    ) -> HashMap<String,  f64> {
        let quantity_by_transaction_by_currency=self.get_quantity_by_transaction_type_by_asset();
        quantity_by_transaction_by_currency.into_iter().map(|(currency,transaction)| {
           (currency ,(transaction.get(&TransactionType::BUY).unwrap() - transaction.get(&TransactionType::SELL).unwrap() ))
        }).collect()
    }
    pub fn get_assets(
        &self,
     )  -> Vec<String> {
        let current_quantity_by_currency=self.get_current_quantity_by_asset();
        current_quantity_by_currency.into_iter().filter(|(_,value)| *value >0.0).map(|(currency,_)| currency).collect()
    }

    pub fn  get_tickers(&self) -> HashSet<Ticker> {
        self.transactions.clone().into_iter().map(|t|t.ticker).collect()

    }
    pub fn get_total_cost_by_transaction_type_by_currency(
        &self,
    ) -> HashMap<String, HashMap<TransactionType, f64>> {
        self.transactions
            .clone()
            .into_iter()
            .group_by(|transaction| transaction.asset.clone())
            .into_iter()
            .map(|(asset, transaction)| {
                (
                    asset,
                    transaction
                        .into_iter()
                        .group_by(|transaction| transaction.transaction_type.clone())
                        .into_iter()
                        .map(|(transactiontype, transaction1)| {
                            (
                                transactiontype,
                                transaction1
                                    .map(|transac| {
                                        transac.currency_quantity + transac.currency_fees_quantity
                                    })
                                    .sum(),
                            )
                        })
                        .collect(),
                )
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub fn new_bitcoin_buy_transaction_exchange_euros(
        asset_quantity: f64,
        currency_quantity: f64,
        currency_fees_quantity: f64,
    ) -> Transaction {
        let exchange_spot_trading = String::from("exchange-spot-trading");
        let exchange_wallet = String::from("my-exchange-wallet");
        let now = Utc::now();
        Transaction {
            source: exchange_spot_trading,
            destination: exchange_wallet,
            transaction_type: TransactionType::BUY,
            asset: "bitcoin".to_string(),
            ticker: "BTC-USD".to_string(),
            asset_quantity: asset_quantity,
            currency: "euros".to_string(),
            currency_quantity: currency_quantity,
            currency_fees: "euros".to_string(),
            currency_fees_quantity: currency_fees_quantity,
            sent_date: now,
            received_date: now,
        }
    }
    #[test]
    fn get_quantity_by_transaction_type_by_asset() {
        let transaction1 = new_bitcoin_buy_transaction_exchange_euros(0.4, 100.0, 2.0);
        let transaction2 = new_bitcoin_buy_transaction_exchange_euros(0.6, 100.0, 2.0);
        let transactions = Transactions {
            transactions: vec![transaction1, transaction2],
        };
        let quantity_by_transaction_type_by_currency = transactions.get_quantity_by_transaction_type_by_asset();
        assert_eq!(
            quantity_by_transaction_type_by_currency
                .get("bitcoin")
                .unwrap()
                .get(&TransactionType::BUY),
            Some(&1.0)
        )
    }
    #[test]
    fn get_total_cost_by_transaction_type_by_currency() {
        let transaction1 = new_bitcoin_buy_transaction_exchange_euros(0.4, 100.0, 2.0);
        let transaction2 = new_bitcoin_buy_transaction_exchange_euros(0.6, 100.0, 2.0);
        let transactions = Transactions {
            transactions: vec![transaction1, transaction2],
        };
        let quantity_by_transaction_type_by_currency = transactions.get_total_cost_by_transaction_type_by_currency();
        assert_eq!(
            quantity_by_transaction_type_by_currency
                .get("bitcoin")
                .unwrap()
                .get(&TransactionType::BUY),
            Some(&204.0)
        )
    }
    #[test]
    fn get_quantity_by_asset() {
        let transaction1 = new_bitcoin_buy_transaction_exchange_euros(1.6, 100.0, 2.0);
        let mut transaction2 = new_bitcoin_buy_transaction_exchange_euros(0.4, 200.0, 2.0);
        transaction2.transaction_type = TransactionType::SELL;
        let transactions = Transactions {
            transactions: vec![transaction1, transaction2],
        };
        let quantity_by_currency = transactions.get_current_quantity_by_asset();
        
        assert_eq!(
            format!("{:.5}",quantity_by_currency
                .get("bitcoin")
                .unwrap()),
                format!("{:.5}", 1.2)
        )
    }
    #[test]
    fn get_assets() {
        let transaction1 = new_bitcoin_buy_transaction_exchange_euros(1.6, 100.0, 3.0);
        let mut transaction2 = new_bitcoin_buy_transaction_exchange_euros(0.4, 200.0, 2.0);
        transaction2.transaction_type = TransactionType::SELL;
        let transactions = Transactions {
            transactions: vec![transaction1, transaction2],
        }; 
        let assets = transactions.get_assets();
        assert_eq!(   *assets.get(0).unwrap(),"bitcoin") 

    }
    #[test]
    fn get_tickers() {
        let transaction1 = new_bitcoin_buy_transaction_exchange_euros(1.6, 100.0, 3.0);
        let mut transaction2 = new_bitcoin_buy_transaction_exchange_euros(0.4, 200.0, 2.0);
        transaction2.transaction_type = TransactionType::SELL;
        let transactions = Transactions {
            transactions: vec![transaction1, transaction2],
        }; 
        let tickers = transactions.get_tickers();
        assert_eq!(   tickers.get("BTC-USD").unwrap(),"BTC-USD") 
    }
}
