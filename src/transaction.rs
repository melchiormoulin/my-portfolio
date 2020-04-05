extern crate chrono;

use itertools::Itertools;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    source: Entity,
    destination: Entity,
    transaction_type: TransactionType,
    asset: Currency,
    asset_quantity: f64,
    currency: Currency,
    currency_quantity: f64,
    currency_fees: Currency,
    currency_fees_quantity: f64,
    sent_date: DateTime<Utc>,
    received_date: DateTime<Utc>,
}
impl Transaction {
    pub fn new_bitcoin_buy_transaction_exchange_euros(
        asset_quantity: f64,
        currency_quantity: f64,
        currency_fees_quantity: f64,
    ) -> Transaction {
        let exchange_spot_trading = Entity {
            name: String::from("exchange-spot-trading"),
        };
        let exchange_wallet = Entity {
            name: String::from("my-exchange-wallet"),
        };
        let now = Utc::now();
        Transaction {
            source: exchange_spot_trading,
            destination: exchange_wallet,
            transaction_type: TransactionType::BUY,
            asset: Currency::BITCOIN,
            asset_quantity: asset_quantity,
            currency: Currency::EUROS,
            currency_quantity: currency_quantity,
            currency_fees: Currency::EUROS,
            currency_fees_quantity: currency_fees_quantity,
            sent_date: now,
            received_date: now,
        }
    }
}

#[derive(Serialize, Copy, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    BUY,
    SELL,
    TRANSFER,
}
#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    BITCOIN,
    ETHEREUM,
    XRP,
    EUROS,
}
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Entity {
    name: String,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    quantity_by_asset: HashMap<Currency, HashMap<TransactionType, f64>>,
    total_asset_cost: HashMap<Currency, HashMap<TransactionType, f64>>,
}
impl Wallet {
    pub fn new(transactions: Transactions) -> Wallet {
        Wallet {
            quantity_by_asset: transactions.get_quantity_by_asset(),
            total_asset_cost: transactions.get_total_asset_cost(),
        }
    }
}
impl Transactions {
    pub fn get_quantity_by_asset(&self) -> HashMap<Currency, HashMap<TransactionType, f64>> {
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
    pub fn get_total_asset_cost(&self) -> HashMap<Currency, HashMap<TransactionType, f64>> {
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
    #[test]
    fn get_quantity_by_asset() {
        let transaction1 = Transaction::new_bitcoin_buy_transaction_exchange_euros(0.4, 100.0, 2.0);
        let transaction2 = Transaction::new_bitcoin_buy_transaction_exchange_euros(0.6, 100.0, 2.0);
        let transactions = Transactions {
            transactions: vec![transaction1, transaction2],
        };
        let quantity_by_asset = transactions.get_quantity_by_asset();
        assert_eq!(
            quantity_by_asset
                .get(&Currency::BITCOIN)
                .unwrap()
                .get(&TransactionType::BUY),
            Some(&1.0)
        )
    }
    #[test]
    fn get_total_asset_cost() {
        let transaction1 = Transaction::new_bitcoin_buy_transaction_exchange_euros(0.4, 100.0, 2.0);
        let transaction2 = Transaction::new_bitcoin_buy_transaction_exchange_euros(0.6, 100.0, 2.0);
        let transactions = Transactions {
            transactions: vec![transaction1, transaction2],
        };
        let quantity_by_asset = transactions.get_total_asset_cost();
        assert_eq!(
            quantity_by_asset
                .get(&Currency::BITCOIN)
                .unwrap()
                .get(&TransactionType::BUY),
            Some(&204.0)
        )
    }
}