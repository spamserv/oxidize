# Oxidize

An attempt to implement simple blockchain using Rust, while learning Rust.

<p align="center">
    <img src="https://github.com/spamserv/oxidize/blob/main/logo.png?raw=true" alt="drawing" width="300"/>
</p>

## Blockchain Project Development Roadmap

A detailed checklist to track the progress:.

### ✅ Completed Milestones
- [x] Implement blockchain structure with vector of blocks.
- [x] Add block header (timestamp, previous_hash, current_hash, nonce, difficulty).
- [x] Create block body with transactions (inputs, outputs, metadata).
- [x] Generate the genesis block.

### 🛠️ Next Steps

Here is the current state of the blockchain:

#### 1. Consensus Mechanism
- [x] Implement a basic consensus algorithm (e.g., Proof of Work or Proof of Stake).
- [x] Create a mining function to solve nonce for PoW.
- [ ] Develop a staking mechanism for PoS, if chosen.

#### 1.1 Initial supply
- [x] Pre-mining / Developer fund - creator gets the initial set of coins to himself for development
- [ ] Initial Coin Offering (ICO) - issuing / selling tokens to early adopters

#### 2. Mine Fee
- [x] Send mining fee for each new block mined to Node's wallet
- [ ] UTXO maturity period for newly created coinbase mining fee

#### 3. Faucets
- [ ] Distribute small amounts of cryptocurrencies for free to help users start interacting, e.g. when a Node joins for the first time.

#### 2. Mempool
- [ ] Implement mempool for the unprocessed (pending) transactions.
- [ ] Once the block is mined, remove transactions from mempool and update transaction status.

#### 3.1. Transaction Flexibility
- [ ] Define Transaction and its usage
- [ ] Decide on block size, based on the usage of blockchain
- [ ] Decide if block will be flexible in size (changeable) or fixed
- [ ] Add transaction fee

#### 3.2. Transaction Validation
- [ ] Implement transaction validation (e.g., check inputs vs. outputs).
- [ ] Add digital signatures for transactions to ensure authenticity.
- [ ] Implement which transaction go into block, and which don't based on transaction fee

#### 4. Wallets
- [x] Implement Wallet & Address creation
- [ ] Implement send of value functionality
- [ ] Get balance & transaction history for all accounts

#### 5. Networking and Node Communication
- [x] Build TCP client/server communication for wallet and node communication.
- [ ] Build a peer-to-peer (P2P) network for node communication.
- [ ] Implement block and transaction propagation among nodes.
- [ ] Develop synchronization for consistent blockchain copies across nodes.

#### 6. Block Verification
- [x] Create a function to verify block integrity (hash, timestamp, difficulty).
- [x] Verify single block validity.
- [x] Verify validity of part of the chain.
- [x] Verify validity of part of the full chain.
- [ ] Implement block validation rules for consensus enforcement.

#### 7. Block Mining
- [ ] Mine a block based on a condition, e.g. block is minimum 70% of its limit
- [x] Add block mining fee

#### 8. Blockchain State Management
- [ ] Design a system to track user balances and UTXOs.
- [ ] Update blockchain state after adding new blocks.

#### 9. Merkle Tree Structure
- [ ] Implement Merkle trees for block transactions.
- [ ] Integrate Merkle root in block headers for transaction verification.

#### 10. User Interface (Optional)
- [ ] Create a CLI interface for blockchain interaction.
- [ ] Build a simple web interface for user interactions like sending transactions and viewing blocks.

#### 11. Security Features
- [ ] Add measures to prevent double-spending.
- [ ] Implement protections against replay attacks.
- [ ] Include time constraints for block mining to prevent stale blocks.

#### 12. Logging and Monitoring
- [ ] Add detailed logging for blockchain events.
- [ ] Implement monitoring tools to track blockchain performance metrics.

#### 13. Advanced Features
- [ ] Introduce smart contract functionality for extended capabilities.
- [ ] Research scalability solutions (e.g., sharding, layer-2 technologies).

#### 14. Airdrop
- [ ] TBD

## Oxidize - Project Explanation

This is a Rust-based blockchain aimed at learning purposes. I created it based on different resources and techniques and I used what I thought is correct and interesting.

Down is the explanation of each section of a blockchain.

## Oxidize structure

### Blockchain (Node)

### Wallet

### Address

### Helpers

### TransactionManager

### Config

## Oxidize Features

#### 1. Consensus Mechanism

#### 2.1. Transaction Flexibility

#### 2.2. Transaction Validation

#### 3. Wallets
Wallet is able to:
- create new accounts
- connect to blockchain node
- exchange messages, such as:
  - get balance
  - get transaction history
  - ping

#### 4. Networking and Node Communication

0. **WebSocket Server**:
There is a bare WebSocket implementation that listens to new connections and spawns a new tokio task for each new connection. It should be spawned in a separate thread not to block the main thread.
WebSocket server accepts address and message handler as parameters in `.run()` function that allows a separation of business logic from the actual WS server.
WebSocket server supports:
- [x] send
- [x] broadcast
- [ ] anycast (TBD)

1. **Blockchain Listener**:
Blockchain node has a BlockchainListener that is a wrapper around my implementation of WebSocketServer using Tokio library.
Current intention is to use *Blockchain Listener* for communication between `Wallets` and `Blockchain Node` such as to get balance, get transaction history, ping and similar functionalities.

2. **Inter-Node Communication**:
- TBD

#### 5. Block Verification

1. **Single Block Verification**: 
Block is a fundamental part of the blockchain. Its main purpose is to carry and store transactions. Blockchains building blocks are actually - blocks. Each block has a header and body. In order to verify a block, there are several checks implemented, which can be found in the `enum BlockValidationError`:

```
enum BlockValidationError {
    #[error("Block not found with the specified hash")]
    BlockNotFound,
    #[error("Blockchain must have at least 2 blocks")]
    InsufficientBlocks,
    #[error("Invalid block hash format or value")]
    InvalidHash,
    #[error("Previous block not found in chain")]
    PreviousBlockNotFound,
    #[error("Previous hash mismatch")]
    PreviousHashMismatch,
    #[error("Block timestamp must be greater than previous block")]
    InvalidTimestamp,
}
```

2. **Part & Full Chain Validity**: 
There should be an option, other than to check a single disparity, to check both:
 - the full chain validity
 - validity on a certain range.

3. **Validation rules for consensus enforcement**:
Beyond cryptographic integrity, there should be a consenus mechanism to check for:
  - valid transactions - each transaction in the block must adhere to certain rules
  - PoW/Stake validation - at the moment, there is only a simple PoW mechanism with simple block difficulty for local development.
  - block creation rules - mechanism to understand who and when can create blocks and what types of transaction to include
  - network rules - enforce particuolar network structure with *block size limit* and *transaciton format*

#### 6. Blockchain State Management

#### 7. Merkle Tree Structure

#### 8. User Interface (Optional)

#### 9. Security Features

#### 10. Logging and Monitoring

#### 11. Advanced Features

## Mods / Crates

### blockchain
Consists of blockchain, block, blockchain_listener.

### config
Includes configuration constants.

### transaction
Consists of Transaction definition, Header, Body (Input, Output, Metadata), and has blueprint for TransactionManager that will create transactions and track status of a transaction.

### utils
Has helper functions for hash and transactions.

### wallet
Includes Wallet, WalletClient to connect to the Node, Accounts, Addresses, Wallet Message types to exchange with the Node. 

### websockets
Includes WebSocketClient and WebSocketServer implementation for various stuff.

## Tests
At one point, you should be able to run tests per crate. TBD

So far so good. Probably ton of refactoring coming in place.

---
