Handle your wallet based  your asset transactions

Based on transactions json file  ( see examples/wallet.json ) computes :
- get_quantity_by_transaction_type_by_currency
- get_total_cost_by_transaction_type_by_currency


build it:

```docker build . -t my-portfolio```

run it: 

```docker run -v $(pwd)/examples/:/workspace/examples my-portfolio```