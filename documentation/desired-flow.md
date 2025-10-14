# Blockchain Node Initiation

START
 ├── Load config
 ├── Load blockchain (or create genesis)
 ├── Connect to peers
 ├── Sync missing blocks
 ├── Listen for new transactions
 │     └── Validate + store in mempool
 ├── If miner: create & broadcast new blocks
 ├── If receive block: validate + append
 ├── Update state + persist
 └── Repeat forever

 # CLI tool to pick next steps

 # Modules

```
network: peer discovery, messaging

chain: block/transaction structs, storage, validation

mempool: temporary transaction store

miner: block creation logic

state: balances, UTXO or account model

crypto: hashing, signing, verifying
```