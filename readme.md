# _Noob_ blockchain 🦀 implementation

I realized I'm noob... Just wanna implement some simple stuffs for learning purpose to my first ever Rust 🦀 project!

### Index

- [x] Noob BlockChain

- [ ] Noob Starknet
  - [x] Blockheader : [Starknet book](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/header/) block header
  - [x] Transaction Type : [Starknet book](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/) Invoke / Declare type
  - [x] Transaction Executor : `Blockifier` for execution
  - [ ] JSON RPC server : To enable user to send tx
  - [ ] State DB : [Starknet Book](https://docs.starknet.io/documentation/architecture_and_concepts/State/starknet-state/)
  - [ ] Transaction Cycle
    - [ ] The sequencer and executes **validate** of sent tx, If the transaction was valid, the **execute** function is called, charges the fee : `tx.execute in blockifier`
    - [ ] If **execute** ran successfully, the sequencer includes the transaction in the block (Pending Block)
    - [ ] Close the block and save in storage ( Finalized Block )

### Reference

- foundry
- katana
- starknetbook

### Learning

- `Blockifier` and `Starknet-rs` using different type. Tip for this, atm if you want to execute, you need to use `Blockifier` but except for that following `Starknet-rs` type would be much easier
