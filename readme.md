# _Noob_ blockchain 🦀 implementation

I realized I'm noob... Just wanna implement some simple stuffs for learning purpose to my first ever Rust 🦀 project!

### Index

- [x] Noob BlockChain

- [ ] Noob Starknet

  - [x] Blockheader : [Starknet book](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/header/) block header
  - [x] Transaction Type : [Starknet book](https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/) Invoke / Declare type
  - [x] Transaction Executor : `Blockifier` for execution
  - [x] State DB : [Blockifier Simple DB implementation](https://github.com/starkware-libs/blockifier/blob/3c8ee7f541db035b49fcfb203aa85f8b0b6b42e5/crates/blockifier/src/test_utils.rs#L108)/extra references :
        [Starknet Book](https://docs.starknet.io/documentation/architecture_and_concepts/State/starknet-state/), [Blockifier State implementation](https://github.com/starkware-libs/blockifier/blob/main/crates/blockifier/src/state/cached_state.rs), [Katana State implementation](https://github.com/dojoengine/dojo/blob/main/crates/katana/core/src/backend/state.rs)
  - [ ] Transaction Cycle
    - [ ] Execute Transaction
  - [ ] JSON RPC server : To enable user to send tx

### Reference

- foundry
- katana
- blockifier
- starknetbook

### Learning

- `Blockifier` and `Starknet-rs` using different type. Tip for this, atm if you want to execute, you need to use `Blockifier` but except for that following `Starknet-rs` type would be much easier
