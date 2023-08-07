use starknet::core::{crypto::compute_hash_on_elements, types::FieldElement};

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub parent_block_hash: FieldElement,
    pub block_number: u64,
    pub global_state_root: FieldElement,
    pub sequencer_address: FieldElement,
    pub block_timestamp: u64,
    pub transaction_count: u64,
    pub transaction_commitment: FieldElement,
    pub event_count: u64,
    pub event_commitment: FieldElement,
    pub protocol_version: u64,
    pub gas_price: FieldElement,
}

impl BlockHeader {
    pub fn new(
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

    pub fn hash(&self) -> FieldElement {
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
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<FieldElement>,
}
