use chrono::Utc;
use starknet::core::{crypto::compute_hash_on_elements, types::FieldElement};

#[derive(Debug, Clone)]
struct BlockHeader {
    parent_block_hash: FieldElement,
    block_number: u64,
    global_state_root: FieldElement,
    sequencer_address: FieldElement,
    block_timestamp: u64,
    transaction_count: u64,
    transaction_commitment: FieldElement,
    event_count: u64,
    event_commitment: FieldElement,
    protocol_version: u64,
    gas_price: FieldElement,
}

impl BlockHeader {
    fn new(
        parent_block_hash: FieldElement,
        block_number: u64,
        global_state_root: FieldElement,
        sequencer_address: FieldElement,
        block_timestamp: u64,
        transaction_count: u64,
        transaction_commitment: FieldElement,
        event_count: u64,
        event_commitment: FieldElement,
        protocol_version: u64,
        gas_price: FieldElement,
    ) -> Self {
        Self {
            parent_block_hash,
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
        }
    }

    fn hash(&self) -> FieldElement {
        compute_hash_on_elements(&vec![
            self.block_number.into(),
            self.global_state_root,
            self.sequencer_address,
            self.block_timestamp.into(),
            self.transaction_count.into(),
            self.transaction_commitment,
            self.event_count.into(),
            self.event_commitment,
            FieldElement::ZERO,
            FieldElement::ZERO,
            self.protocol_version.into(),
        ])
    }
}

#[derive(Debug, Clone)]
struct Block {
    header: BlockHeader,
    transactions: Vec<FieldElement>,
}

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
