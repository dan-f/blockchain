use block::{Block, GenesisBlock};
use list::List;

const GENESIS_HASH: &str = &"001d972e7545f7cbb6d0497ddc47b8eece30a49ee3213fb36e1e851d9baae1";

fn to_hex_string(hash: &[u8]) -> String {
    let strs: Vec<String> = hash.iter().map(|byte| format!("{:x?}", byte)).collect();
    strs.join("")
}

pub struct BlockChain {
    pub genesis: GenesisBlock,
    pub genesis_hash: Vec<u8>,
    pub blocks: List<Block>,
}

pub fn make_genesis() -> (GenesisBlock, Vec<u8>) {
    let genesis = GenesisBlock {
        block_num: 0,
        timestamp: 1528749153,
        nonce: 151771,
        data: String::from("Coded by Dan & Henry at RC 6/11/2018!"),
    };
    let genesis_hash = genesis.hash().expect("Failed to hash genesis block!");
    if to_hex_string(&genesis_hash) != GENESIS_HASH {
        panic!("Hash of genesis block is not as expected");
    }
    (genesis, genesis_hash)
}

impl BlockChain {
    pub fn new() -> Self {
        let (genesis, genesis_hash) = make_genesis();
        BlockChain {
            genesis,
            genesis_hash,
            blocks: List::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_genesis_block() {
        let (genesis, genesis_hash) = make_genesis();
        assert_eq!(genesis.data, "Coded by Dan & Henry at RC 6/11/2018!");
        assert_eq!(&genesis_hash[0..2], [0, 0]);
        assert_eq!(to_hex_string(&genesis_hash[..]), GENESIS_HASH);
    }

    #[test]
    fn new_blockchain_uses_genesis_block() {
        let chain = BlockChain::new();
        assert_eq!(chain.genesis.hash().unwrap(), chain.genesis_hash);
        assert_eq!(to_hex_string(&chain.genesis_hash), GENESIS_HASH);
    }
}
