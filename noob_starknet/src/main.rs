use block::{Block, BlockHeader};
use blockifier::{
    block_context::BlockContext,
    state::cached_state::{CachedState, GlobalContractCache},
    transaction::{
        account_transaction::AccountTransaction,
        transaction_execution::Transaction as ExecutionTransaction,
    },
};
use chrono::Utc;
use executor::execute_transaction;
use starknet::core::types::FieldElement;
use starknet_api::core::ContractAddress;
use starknet_api::core::PatriciaKey;
use starknet_api::hash::StarkHash;
use starknet_api::{
    block::{BlockNumber, BlockTimestamp},
    core::ChainId,
    transaction::Transaction as ApiTransaction,
};
use starknet_api::{contract_address, patricia_key};
use tracing::trace;

use crate::state::DictStateReader;

pub mod block;
pub mod executor;
pub mod state;
pub mod transaction;

fn create_genesis_block() -> Block {
    Block {
        header: BlockHeader::new(
            FieldElement::ZERO,
            0,
            FieldElement::ZERO,
            FieldElement::ZERO,
            0,
            0,
            FieldElement::ZERO,
            0,
            FieldElement::ZERO,
            0,
            FieldElement::ZERO,
        ),
        transactions: vec![],
    }
}



fn create_new_block(
    prev_block: &Block,
    transaction: ExecutionTransaction,
    charge_fee: bool,
    data: String,
) -> Block {
    let prev_block_hash = prev_block.header.hash();
    let block_number = prev_block.header.block_number + 1;
    let global_state_root = FieldElement::ZERO;
    let sequencer_address = FieldElement::ZERO;
    let block_timestamp = Utc::now().timestamp() as u64;
    let transaction_count = 0;
    let transaction_commitment = FieldElement::ZERO;
    let event_count = 0;
    let event_commitment = FieldElement::ZERO;
    let protocol_version = 0;
    let gas_price = FieldElement::ZERO;

    pub const CURRENT_BLOCK_NUMBER: u64 = 2000;
    pub const TEST_SEQUENCER_ADDRESS: &str = "0x1000";
    pub const TEST_ERC20_CONTRACT_ADDRESS: &str = "0x1001";
    pub const DEFAULT_GAS_PRICE: u128 = 100 * u128::pow(10, 9); // Given in units of wei.

    let mock_block_context: BlockContext = BlockContext {
        chain_id: ChainId("SN_GOERLI".to_string()),
        block_number: BlockNumber(CURRENT_BLOCK_NUMBER),
        block_timestamp: BlockTimestamp::default(),
        sequencer_address: contract_address!(TEST_SEQUENCER_ADDRESS),
        fee_token_address: contract_address!(TEST_ERC20_CONTRACT_ADDRESS),
        vm_resource_fee_cost: Default::default(),
        gas_price: DEFAULT_GAS_PRICE,
        invoke_tx_max_n_steps: 1_000_000,
        validate_max_n_steps: 1_000_000,
        max_recursion_depth: 50,
    };

    let res = execute_transaction(
        transaction,
        &mut CachedState::from(DictStateReader {
            storage_view: Default::default(),
            address_to_nonce: Default::default(),
            address_to_class_hash: Default::default(),
            class_hash_to_class: Default::default(),
            class_hash_to_compiled_class_hash: Default::default(),
        }),
        &mock_block_context,
        charge_fee,
        true,
    );

    match res {
        Ok(execution_info) => {
            trace!("Transaction execution info: {:?}", execution_info);
        }

        Err(err) => {
            trace!("Transaction execution error: {:?}", err);
        }
    }

    let header = BlockHeader::new(
        prev_block_hash,
        CURRENT_BLOCK_NUMBER,
        global_state_root,
        sequencer_address,
        block_timestamp,
        transaction_count,
        transaction_commitment,
        event_count,
        event_commitment,
        protocol_version,
        gas_price,
    );

    let transactions = vec![transaction::Transaction::Invoke(
        transaction::InvokeTransaction {
            status: transaction::TransactionStatus::AcceptedOnL2,
            sender: FieldElement::ZERO,
            call_data: vec![FieldElement::ZERO],
            version: FieldElement::ZERO,
            nonce: FieldElement::ZERO,
            max_fee: FieldElement::ZERO,
            signature: vec![FieldElement::ZERO],
        },
    )];

    Block {
        header,
        transactions,
    }
}


fn main() {
    let mut blockchain: Vec<Block> = vec![create_genesis_block()];
    let mut prev_block = blockchain[0].clone();

    let num_blocks = 10;

    for i in 1..=num_blocks {
        let new_block_data = format!("Block #{} data.", i);
        // TODO: add dummy tx
        let tx
        let new_block = create_new_block(&prev_block, tx, false, new_block_data);
        println!(
            "Block #{} has been added to the blockchain!, {}, {:?}",
            new_block.header.block_number,
            new_block.header.block_timestamp,
            new_block.transactions[0].clone()
        );
        println!("Hash: {}\n", new_block.header.hash());
        blockchain.push(new_block.clone());
        prev_block = new_block;
    }
}
