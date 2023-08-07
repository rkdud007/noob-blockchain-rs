use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Clone)]
struct Block {
    index: u32,
    timestamp: DateTime<Utc>,
    hash: String,
    parent_hash: String,
    data: String,
}

impl Block {
    fn new(index: u32, timestamp: DateTime<Utc>, parent_hash: String, data: String) -> Self {
        let hash = Self::compute_hash(index, timestamp, &parent_hash, &data);
        Block {
            index,
            timestamp,
            hash,
            parent_hash,
            data,
        }
    }

    fn compute_hash(index: u32, timestamp: DateTime<Utc>, parent_hash: &str, data: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, parent_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

fn create_genesis_block() -> Block {
    Block::new(0, Utc::now(), "0".to_owned(), "Genesis Block".to_owned())
}
fn create_new_block(prev_block: &Block, data: String) -> Block {
    let index = prev_block.index + 1;
    let timestamp = Utc::now();
    let parent_hash = prev_block.hash.clone();
    Block::new(index, timestamp, parent_hash, data)
}

fn main() {
    let mut blockchain: Vec<Block> = vec![create_genesis_block()];
    let mut prev_block = blockchain[0].clone();

    let num_blocks = 10;

    for i in 1..=num_blocks {
        let new_block_data = format!("Block #{} data.", i);
        let new_block = create_new_block(&prev_block, new_block_data);
        println!(
            "Block #{} has been added to the blockchain!, {}, {}",
            new_block.index, new_block.timestamp, new_block.data
        );
        println!("Hash: {}\n", new_block.hash);
        blockchain.push(new_block.clone());
        prev_block = new_block;
    }
}
