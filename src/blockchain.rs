use block::{Block, GenesisBlock};
use list::List;
use util;

pub fn is_acceptable_proof_of_work(hash: &[u8]) -> bool {
    hash.len() == 32 && hash[0..2] == [0, 0]
}

#[derive(Debug, PartialEq)]
pub enum Error {
    WrongHash {
        current_top_hash: Vec<u8>,
        claimed_top_hash: Vec<u8>,
    },
    UnacceptableProofOfWork {
        hash: Vec<u8>,
    },
}

pub struct BlockChain {
    pub genesis: GenesisBlock,
    pub genesis_hash: Vec<u8>,
    pub blocks: List<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let genesis = GenesisBlock::new();
        let genesis_hash = genesis.hash();
        BlockChain {
            genesis,
            genesis_hash,
            blocks: List::new(),
        }
    }

    pub fn top_hash(&self) -> Vec<u8> {
        match self.blocks.iter().next() {
            Some(ref most_recent_block) => most_recent_block.hash().unwrap(), // FIXME: replace with slice
            None => self.genesis_hash.clone(), // FIXME: replace with slice
        }
    }

    pub fn mine(&self, data: &str) -> (Block, Vec<u8>) {
        let prev_hash = self.top_hash();
        let mut block = Block {
            block_num: 1 + self.blocks.len() as u64, // magic number 1 accounts for the genesis block, not included in `self.blocks`
            timestamp: util::millis_since_unix_epoch(),
            nonce: 0,
            data: String::from(data),
            prev_hash,
        };
        let mut hash = block.hash().unwrap();
        while !is_acceptable_proof_of_work(&hash) {
            block.nonce += 1;
            hash = block.hash().unwrap();
        }
        (block, hash)
    }

    pub fn add(&mut self, block: Block) -> Result<(), Error> {
        let current_top_hash = self.top_hash();
        if block.prev_hash != current_top_hash {
            return Err(Error::WrongHash {
                current_top_hash: current_top_hash.clone(),
                claimed_top_hash: block.prev_hash.clone(),
            });
        }
        let hash = block.hash().expect("Could not hash proposed block");
        if !is_acceptable_proof_of_work(&hash) {
            return Err(Error::UnacceptableProofOfWork { hash: hash.clone() });
        }
        self.blocks.push(block);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use block::GENESIS_HASH;
    use util;

    #[test]
    fn new_blockchain_uses_genesis_block() {
        let chain = BlockChain::new();
        assert_eq!(chain.genesis.hash(), chain.genesis_hash);
        assert_eq!(util::to_hex_string(&chain.genesis_hash), GENESIS_HASH);
    }

    #[test]
    fn blockchain_can_mine_block() {
        let chain = BlockChain::new();
        let (mined, mined_hash) = chain.mine("a new block!");
        assert_eq!(mined.prev_hash, chain.genesis_hash);
        assert!(is_acceptable_proof_of_work(&mined_hash));
    }

    #[test]
    fn blockchain_can_add_block() {
        let mut chain = BlockChain::new();
        let (mined, mined_hash) = chain.mine("🤷‍♂️");
        assert_eq!(mined.prev_hash, chain.genesis_hash);
        assert!(is_acceptable_proof_of_work(&mined_hash));
        assert!(chain.add(mined).is_ok());
    }

    #[test]
    fn blockchain_will_not_add_block_with_wrong_prev_hash() {
        let mut chain = BlockChain::new();
        let bad_prev_hash = vec![];
        let block = Block {
            block_num: 1,
            timestamp: util::millis_since_unix_epoch(),
            nonce: 0,
            data: String::from("hello"),
            prev_hash: bad_prev_hash.clone(),
        };
        let result = chain.add(block);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err,
            Error::WrongHash {
                current_top_hash: chain.genesis_hash,
                claimed_top_hash: bad_prev_hash,
            }
        );
    }

    #[test]
    fn blockchain_will_not_add_block_with_unacceptable_proof_of_work() {
        let mut chain = BlockChain::new();
        let block = Block {
            block_num: 1,
            timestamp: util::millis_since_unix_epoch(),
            nonce: 0,
            data: String::from("hello"),
            prev_hash: chain.genesis_hash.clone(),
        };
        let hash = block.hash().unwrap();
        assert!(!is_acceptable_proof_of_work(&hash));
        let result = chain.add(block);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, Error::UnacceptableProofOfWork { hash });
    }
}
