use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone)]
struct Block {
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    // Constructor for a new Block
    fn new(data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut block = Block {
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    // Calculates the hash of the block
    fn calculate_hash(&self) -> String {
        let record = format!("{}{}{}", self.timestamp, self.data, self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        let hash = hasher.finalize();
        hex::encode(hash)
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    // Creates a new blockchain with a genesis block
    fn new() -> Self {
        let genesis_block = Block::new("Genesis Block".to_string(), "0".to_string());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    // Adds a new block to the chain
    fn add_block(&mut self, data: String) {
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new(data, previous_hash);
        self.blocks.push(new_block);
    }
}

fn main() {
    let mut chain = Blockchain::new();
    println!("Created a new blockchain.");

    println!("Adding a new block...");
    chain.add_block("Transaction Data 1".to_string());

    println!("Adding another block...");
    chain.add_block("Transaction Data 2".to_string());

    println!("\n--- Blockchain State ---");
    for block in chain.blocks {
        println!("{:?}", block);
    }
    println!("------------------------\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_blockchain() {
        let chain = Blockchain::new();
        assert_eq!(chain.blocks.len(), 1);
        let genesis_block = &chain.blocks[0];
        assert_eq!(genesis_block.data, "Genesis Block");
        assert_eq!(genesis_block.previous_hash, "0");
    }

    #[test]
    fn test_add_block() {
        let mut chain = Blockchain::new();
        chain.add_block("Test Data".to_string());
        assert_eq!(chain.blocks.len(), 2);
        let new_block = &chain.blocks[1];
        let genesis_block = &chain.blocks[0];
        assert_eq!(new_block.data, "Test Data");
        assert_eq!(new_block.previous_hash, genesis_block.hash);
    }
}