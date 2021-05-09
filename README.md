# Handle your wallet based your asset transactions

Based on transactions json file  ( see examples/wallet.json ): 
Print the asset value for each asset in your wallet every 60 sec. The quote is based on yahoo finance.


build it:

```docker build . -t my-portfolio```

run it:

```docker run -v $(pwd)/examples/:/workspace/examples my-portfolio```
