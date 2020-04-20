# Handle your wallet based your asset transactions

Based on transactions json file  ( see examples/wallet.json ) computes :

- The quantity of currency by transaction type.
- The Total cost of currency by transaction type.
- Get current quantity of currency

build it:

```docker build . -t my-portfolio```

run it:

```docker run -v $(pwd)/examples/:/workspace/examples my-portfolio```