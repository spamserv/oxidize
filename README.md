# Oxidize

An attempt to implement simple blockchain using Rust, while learning Rust.

## Blockchain Project Development Roadmap

A detailed checklist to track the progress of building out your blockchain project.

### ✅ Completed Steps
- [x] Implement blockchain structure with vector of blocks.
- [x] Add block header (timestamp, previous_hash, current_hash, nonce, difficulty).
- [x] Create block body with transactions (inputs, outputs, metadata).
- [x] Generate the genesis block.

### 🛠️ Next Steps

#### 1. Consensus Mechanism
- [ ] Implement a basic consensus algorithm (e.g., Proof of Work or Proof of Stake).
- [ ] Create a mining function to solve nonce for PoW.
- [ ] Develop a staking mechanism for PoS, if chosen.

#### 2.1. Transaction Flexibility
- [ ] Define Transaction and its usage
- [ ] Decide on block size, based on the usage of blockchain
- [ ] Decide if block will be flexible in size (changeable) or fixed

#### 2.2. Transaction Validation
- [ ] Implement transaction validation (e.g., check inputs vs. outputs).
- [ ] Add digital signatures for transactions to ensure authenticity.
- [ ] Implement which transaction go into block, and which don't based on transaction fee

#### 3. Networking and Node Communication
- [ ] Build a peer-to-peer (P2P) network for node communication.
- [ ] Implement block and transaction propagation among nodes.
- [ ] Develop synchronization for consistent blockchain copies across nodes.

#### 4. Block Verification
- [ ] Create a function to verify block integrity (hash, timestamp, difficulty).
- [ ] Implement block validation rules for consensus enforcement.

#### 5. Blockchain State Management
- [ ] Design a system to track user balances and UTXOs.
- [ ] Update blockchain state after adding new blocks.

#### 6. Merkle Tree Structure
- [ ] Implement Merkle trees for block transactions.
- [ ] Integrate Merkle root in block headers for transaction verification.

#### 7. User Interface (Optional)
- [ ] Create a CLI interface for blockchain interaction.
- [ ] Build a simple web interface for user interactions like sending transactions and viewing blocks.

#### 8. Security Features
- [ ] Add measures to prevent double-spending.
- [ ] Implement protections against replay attacks.
- [ ] Include time constraints for block mining to prevent stale blocks.

#### 9. Logging and Monitoring
- [ ] Add detailed logging for blockchain events.
- [ ] Implement monitoring tools to track blockchain performance metrics.

#### 10. Advanced Features
- [ ] Introduce smart contract functionality for extended capabilities.
- [ ] Research scalability solutions (e.g., sharding, layer-2 technologies).

### Project Structure

TBD, so far so good. Probably ton of refactoring coming in place.

---
