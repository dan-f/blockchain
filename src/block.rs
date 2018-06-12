use crypto;

#[derive(Serialize, Deserialize, Debug)]
pub struct GenesisBlock {
    pub block_num: u64,
    pub timestamp: u64,
    pub nonce: u64,
    pub data: String,
}

impl GenesisBlock {
    pub fn hash(&self) -> Result<Vec<u8>, crypto::Error> {
        crypto::hash(&self)
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
