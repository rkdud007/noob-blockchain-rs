# _Noob_ blockchain 🦀 implementation

I realized I'm noob... Just wanna implement some simple stuffs for learning purpose to my first ever Rust 🦀 project!

### Index

- [x] Noob BlockChain

- [ ] Noob Starknet
  - [x] Blockheader : [Starknet book](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/header/) block header
  - [x] Transaction Type : [Starknet book](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/) Invoke / Declare type
  - [x] Transaction Executor : `Blockifier` for execution
  - [ ] Transaction Cycle
    - [ ] Send Transaction to mempool
    - [ ] The sequencer selects a transaction from the mempool and executes **validate**
    - [ ] If the transaction was valid, the **execute** function is called
    - [ ] If **execute** ran successfully, the sequencer includes the transaction in the block, charges the fee and proceeds to work on the next transaction (once the block is concluded, it will be sent to the prover)

### Reference

- foundry
- katana
- starknetbook

### Learning

- `Blockifier` and `Starknet-rs` using different type. Tip for this, atm if you want to execute, you need to use `Blockifier` but except for that following `Starknet-rs` type would be much easier
