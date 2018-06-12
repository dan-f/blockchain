use crypto;
use util;

const GENESIS_NONCE: u64 = 151771;
pub const GENESIS_HASH: &str = &"001d972e7545f7cbb6d0497ddc47b8eece30a49ee3213fb36e1e851d9baae1";

#[derive(Serialize, Deserialize, Debug)]
pub struct GenesisBlock {
    pub block_num: u64,
    pub timestamp: u64,
    pub nonce: u64,
    pub data: String,
}

impl GenesisBlock {
    pub fn new() -> Self {
        let genesis = GenesisBlock {
            block_num: 0,
            timestamp: 1528749153,
            nonce: GENESIS_NONCE,
            data: String::from("Coded by Dan & Henry at RC 6/11/2018!"),
        };
        let genesis_hash = genesis.hash();
        if util::to_hex_string(&genesis_hash) != GENESIS_HASH {
            panic!("Hash of genesis block is not as expected");
        }
        genesis
    }

    pub fn hash(&self) -> Vec<u8> {
        crypto::hash(&self).expect("Failed to hash genesis block")
    }
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub block_num: u64,
    pub timestamp: u64,
    pub nonce: u64,
    pub data: String,
    pub prev_hash: Vec<u8>,
}

impl Block {
    pub fn hash(&self) -> Result<Vec<u8>, crypto::Error> {
        crypto::hash(&self)
    }

    pub fn mine(&mut self) -> Result<Vec<u8>, crypto::Error> {
        let mut hash = self.hash()?;
        while hash[0..2] != [0, 0] {
            self.nonce += 1;
            hash = self.hash()?;
        }
        Ok(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_a_genesis_block() {
        let genesis = GenesisBlock::new();
        let genesis_hash = genesis.hash();
        assert_eq!(genesis.data, "Coded by Dan & Henry at RC 6/11/2018!");
        assert_eq!(&genesis_hash[0..2], [0, 0]);
        assert_eq!(util::to_hex_string(&genesis_hash[..]), GENESIS_HASH);
    }
}
