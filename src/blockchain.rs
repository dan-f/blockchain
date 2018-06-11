use block::{Block, GenesisBlock};
use crypto;
use list::List;

pub struct BlockChain {
    pub genesis: GenesisBlock,
    pub genesis_hash: Vec<u8>,
    pub blocks: List<Block>,
}

pub fn make_genesis() -> Result<(GenesisBlock, Vec<u8>), crypto::Error> {
    let mut genesis = GenesisBlock {
        block_num: 0,
        timestamp: 1528749153,
        nonce: 0,
        data: String::from("Coded by Dan & Henry at RC 6/11/2018!"),
    };
    let mut genesis_hash = genesis.hash()?;
    while genesis_hash[0..2] != [0, 0] {
        genesis.nonce += 1;
        genesis_hash = genesis.hash()?;
    }
    Ok((genesis, genesis_hash))
}

impl BlockChain {
    pub fn new() -> Self {
        let (genesis, genesis_hash) = make_genesis().expect("Genesis block failed to hash!");
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
        let (genesis, genesis_hash) = make_genesis().unwrap();
        assert_eq!(genesis.data, "Coded by Dan & Henry at RC 6/11/2018!");
        assert_eq!(&genesis_hash[0..2], [0, 0]);
    }
}
