use std::ops::RangeBounds;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct LockAccount {
    pub buyer: Pubkey, // 32 bytes
    pub amount: u64,   // 8
}

impl LockAccount {
    pub const LEN: usize = 32 + 8; // 40 bytes total

    /// Unpack from slice
    pub fn unpack_from_slice(src: &[u8]) -> Result<Self, std::io::Error> {
        Self::try_from_slice(src)
    }

    /// Pack into slice
    pub fn pack_into_slice(&self, dst: &mut [u8]) -> Result<(), std::io::Error> {
        let encoded = self.try_to_vec()?;
        dst[..encoded.len()].copy_from_slice(&encoded);
        Ok(())
    }
}
