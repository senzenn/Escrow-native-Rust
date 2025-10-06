use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum EscrowInstruction {
    /// Initialize a new escrow
    /// Accounts expected:
    /// 0. [signer, writable] Buyer account
    /// 1. [writable] Escrow PDA account
    /// 2. [] Seller account
    /// 3. [] System program
    InitializeEscrow {
        amount: u64,
        escrow_id: Vec<u8>,
    },

    /// Release funds to seller
    /// Accounts expected:
    /// 0. [signer, writable] Buyer account
    /// 1. [writable] Escrow PDA account
    /// 2. [writable] Seller account
    ReleaseFunds {
        escrow_id: Vec<u8>,
    },

    /// Cancel escrow and refund buyer
    /// Accounts expected:
    /// 0. [signer, writable] Buyer account
    /// 1. [writable] Escrow PDA account
    CancelEscrow {
        escrow_id: Vec<u8>,
    },
}

impl EscrowInstruction {
    /// Unpack instruction from bytes
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = InitializeEscrowPayload::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::InitializeEscrow {
                    amount: payload.amount,
                    escrow_id: payload.escrow_id,
                }
            }
            1 => {
                let payload = ReleaseFundsPayload::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::ReleaseFunds {
                    escrow_id: payload.escrow_id,
                }
            }
            2 => {
                let payload = CancelEscrowPayload::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::CancelEscrow {
                    escrow_id: payload.escrow_id,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

// Helper structs for deserialization
#[derive(BorshDeserialize, BorshSerialize)]
struct InitializeEscrowPayload {
    amount: u64,
    escrow_id: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct ReleaseFundsPayload {
    escrow_id: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CancelEscrowPayload {
    escrow_id: Vec<u8>,
}
