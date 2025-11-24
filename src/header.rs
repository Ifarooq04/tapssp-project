use crate::error::{LockBoxError, LockBoxResult};
use std::io::{Read, Write};

pub struct Header {
    pub nonce: Vec<u8>,
    pub wrapped_key: Vec<u8>,
}

impl Header {
    pub const MAGIC: [u8; 4] = *b"LBX1";
    pub const VERSION: u8 = 1;

    pub fn write_to<W: Write>(&self, mut w: W) -> LockBoxResult<()> {
        w.write_all(&Self::MAGIC)?;
        w.write_all(&[Self::VERSION])?;

        if self.nonce.len() > u8::MAX as usize {
            return Err(LockBoxError::Format("Nonce too long".into()));
        }

        if self.wrapped_key.len() > u16::MAX as usize {
            return Err(LockBoxError::Format("Wrapped key too long".into()));
        }

        w.write_all(&[self.nonce.len() as u8])?;
        let key_len = self.wrapped_key.len() as u16;
        w.write_all(&key_len.to_be_bytes())?;

        w.write_all(&self.nonce)?;
        w.write_all(&self.wrapped_key)?;
        Ok(())
    }

    pub fn read_from<R: Read>(mut r: R) -> LockBoxResult<Header> {
        let mut magic = [0u8; 4];
        r.read_exact(&mut magic)?;
        if magic != Self::MAGIC {
            return Err(LockBoxError::Format("Bad file header".into()));
        }

        let mut version = [0u8; 1];
        r.read_exact(&mut version)?;
        if version[0] != Self::VERSION {
            return Err(LockBoxError::Format("Unsupported version".into()));
        }

        let mut nonce_len = [0u8; 1];
        r.read_exact(&mut nonce_len)?;
        let nonce_len = nonce_len[0] as usize;

        let mut wrapped_len_bytes = [0u8; 2];
        r.read_exact(&mut wrapped_len_bytes)?;
        let wrapped_len = u16::from_be_bytes(wrapped_len_bytes) as usize;

        let mut nonce = vec![0u8; nonce_len];
        r.read_exact(&mut nonce)?;

        let mut wrapped_key = vec![0u8; wrapped_len];
        r.read_exact(&mut wrapped_key)?;

        Ok(Header { nonce, wrapped_key })
    }
}
