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
    fn hash(&self, buf: &mut [u8; 32]) -> Result<(), Error> {
        let v = digest(Algorithm::SHA256, self.prev_hash);
        if v.len() != 32 {
            return Err(Error::BadHash);
        }
        let mut i: usize = 0;
        for byte in v.iter() {
            buf[i] = *byte;
            i += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let block = Block {
        prev_hash: b"foo bar",
    };
    let mut digest: [u8; 32] = [0; 32];
    block.hash(&mut digest)?;
    println!("Block hash: {:?}", digest);
    Ok(())
}
