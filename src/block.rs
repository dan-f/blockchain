use bincode::{self, serialize};
use crypto_hash::{digest, Algorithm};

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub timestamp: u64,
    pub prev_hash: Vec<u8>,
}

#[derive(Debug)]
pub enum Error {
    CouldNotSerialize(bincode::Error),
}

impl Block {
    pub fn hash(&self) -> Result<Vec<u8>, Error> {
        match serialize(self) {
            Ok(v) => Ok(digest(Algorithm::SHA256, &v)),
            Err(e) => Err(Error::CouldNotSerialize(e)),
        }
    }
}
