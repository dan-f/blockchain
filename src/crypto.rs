use bincode::{self, serialize};
use crypto_hash::{digest, Algorithm};
use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    CouldNotSerialize(bincode::Error),
}

pub fn hash<T>(data: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    match serialize(data) {
        Ok(v) => Ok(digest(Algorithm::SHA256, &v)),
        Err(e) => Err(Error::CouldNotSerialize(e)),
    }
}
