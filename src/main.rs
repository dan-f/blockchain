extern crate crypto_hash;

use crypto_hash::{digest, Algorithm};

struct Block<'a> {
    prev_hash: &'a [u8],
}

#[derive(Debug)]
enum Error {
    BadHash,
}

impl<'a> Block<'a> {
    fn hash(&self) -> Result<[u8; 32], Error> {
        let v = digest(Algorithm::SHA256, self.prev_hash);
        if v.len() != 32 {
            return Err(Error::BadHash);
        }
        let mut digest: [u8; 32] = [0; 32];
        let mut i: usize = 0;
        for byte in v.iter() {
            digest[i] = *byte;
            i += 1;
        }
        Ok(digest)
    }
}

fn main() -> Result<(), Error> {
    let block = Block {
        prev_hash: b"foo bar",
    };
    let digest = block.hash()?;
    println!("Block hash: {:?}", digest);
    Ok(())
}
