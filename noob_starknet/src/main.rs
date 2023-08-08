use block::{Block, BlockHeader};
use blockifier::{
    block_context::BlockContext,
    execution::contract_class::{ContractClass, ContractClassV0},
    state::{
        cached_state::{CachedState, GlobalContractCache},
        state_api::StateReader,
    },
    transaction::{
        account_transaction::AccountTransaction,
        transaction_execution::Transaction as ExecutionTransaction,
        transactions::DeclareTransaction,
    },
};
use chrono::Utc;
use executor::execute_transaction;
use serde_json::json;
use starknet::core::types::FieldElement;
use starknet_api::transaction::{
    Calldata, DeclareTransaction as DeclareApiTransaction, DeclareTransactionV0V1, Fee,
    InvokeTransaction, InvokeTransactionV1, TransactionHash, TransactionSignature,
};
use starknet_api::{
    block::{BlockNumber, BlockTimestamp},
    core::ChainId,
    transaction::Transaction as ApiTransaction,
};
use starknet_api::{contract_address, patricia_key};
use starknet_api::{core::ContractAddress, state::StorageKey};
use starknet_api::{core::Nonce, hash::StarkHash};
use starknet_api::{
    core::{ClassHash, CompiledClassHash, PatriciaKey},
    hash::StarkFelt,
    stark_felt,
};
use tracing::trace;

use crate::constants::UDC_CONTRACT;
use crate::state::DictStateReader;

pub mod block;
pub mod constants;
pub mod executor;
pub mod state;

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
    global_state: DictStateReader,
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
        &mut CachedState::from(global_state),
        &mock_block_context,
        charge_fee,
        true,
    );

    println!("Transaction: {:?}", res);

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

    let transactions = vec![];

    Block {
        header,
        transactions,
    }
}

fn deploy_universal_deployer_contract() -> DictStateReader {
    let class_hash = ClassHash(stark_felt!("0x1"));
    let address = ContractAddress(patricia_key!("0x1"));
    let storage_key = StorageKey(patricia_key!("0x77"));
    let storage_val = stark_felt!("0x66");
    let contract = (*UDC_CONTRACT).clone();
    let compiled_hash = CompiledClassHash(class_hash.0);

    let mut state = DictStateReader::default();
    state
        .storage_view
        .insert((address, storage_key), storage_val);
    state.address_to_nonce.insert(address, Nonce::default());
    state
        .address_to_class_hash
        .insert(address, class_hash.clone());
    state.class_hash_to_class.insert(class_hash, contract);
    state
        .class_hash_to_compiled_class_hash
        .insert(class_hash, compiled_hash);
    state
}

fn main() {
    let a = ContractAddress(patricia_key!("0x1"));
    let mut blockchain: Vec<Block> = vec![create_genesis_block()];

    let prev_block = blockchain.pop().unwrap();

    let global_state = deploy_universal_deployer_contract();

    let new_block_data = format!("Block #{} data.", 1);
    // TODO: add dummy tx
    let tx = ExecutionTransaction::AccountTransaction(
        blockifier::transaction::account_transaction::AccountTransaction::Declare(
            DeclareTransaction::new(
                DeclareApiTransaction::V0(DeclareTransactionV0V1 {
                    nonce: Nonce(1u8.into()),
                    sender_address: a,
                    signature: TransactionSignature::default(),
                    class_hash: ClassHash(stark_felt!("0x1")),
                    max_fee: Fee::default(),
                }),
                TransactionHash(stark_felt!("0x6969")),
                (*UDC_CONTRACT).clone(),
            )
            .unwrap(),
        ),
    );

    let new_block = create_new_block(&prev_block, tx, false, new_block_data, global_state);
    println!(
        "Block #{} has been added to the blockchain!, {}, {:?}",
        new_block.header.block_number, new_block.header.block_timestamp, new_block
    );
    println!("Hash: {}\n", new_block.header.hash());
}
