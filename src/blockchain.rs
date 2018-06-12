use block::{Block, GenesisBlock};
use list::List;

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
}
