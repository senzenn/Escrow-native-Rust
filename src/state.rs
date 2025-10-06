use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, borsh::BorshDeserialize, Debug, Clone)]
pub struct EscrowAccount {
    pub is_initialized: bool, // 1 byte
    pub buyer: Pubkey,        // 32 bytes
    pub seller: Pubkey,       // 32 bytes
    pub amount: u64,          // 8 bytes
}

impl EscrowAccount {
    pub const LEN: usize = 1 + 32 + 32 + 8; // 73 bytes total

    /// Unpack from slice
    pub fn unpack_from_slice(src: &[u8]) -> Result<Self, std::io::Error> {
        borsh::BorshDeserialize::try_from_slice(src)
    }

    /// Pack into slice
    pub fn pack_into_slice(&self, dst: &mut [u8]) -> Result<(), std::io::Error> {
        let encoded = self.try_to_vec()?;
        dst[..encoded.len()].copy_from_slice(&encoded);
        Ok(())
    }
}
