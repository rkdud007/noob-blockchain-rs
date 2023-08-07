use block::{Block, BlockHeader};
use chrono::Utc;
use starknet::core::types::FieldElement;

pub mod block;

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

fn create_new_block(prev_block: &Block, data: String) -> Block {
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

    let header = BlockHeader::new(
        prev_block_hash,
        block_number,
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

fn main() {
    let mut blockchain: Vec<Block> = vec![create_genesis_block()];
    let mut prev_block = blockchain[0].clone();

    let num_blocks = 10;

    for i in 1..=num_blocks {
        let new_block_data = format!("Block #{} data.", i);
        let new_block = create_new_block(&prev_block, new_block_data);
        println!(
            "Block #{} has been added to the blockchain!, {}",
            new_block.header.block_number, new_block.header.block_timestamp
        );
        println!("Hash: {}\n", new_block.header.hash());
        blockchain.push(new_block.clone());
        prev_block = new_block;
    }
}
