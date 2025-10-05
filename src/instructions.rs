use std::u8;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum VaultInstruction {
    // send sol to vault->
    // expected inputs
    // 0. [signer, writable ] :0-> buyer account
    // 1. [writeable]  Vault pda
    // 2. system program
    SendSol {
        amount: u64,
        lottery_id: Vec<v8>,
    },

    /// 0. `[signer, writable]` Buyer account
    /// 1. `[writable]` Vault PDA account
    Cancel {
        lottery_id: Vec<u8>,
    },

    //  0. `[signer, writable]` Buyer account
    /// 1. `[writable]` Vault PDA account
    CloseVault {
        lottery_id: Vec<u8>,
    },
}

impl VaultInstruction {
    /// unpck/pack to bytes
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = SendSolPayload::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::SendSol {
                    amount: payload.amount,
                    lottery_id: payload.lottery_id,
                }
            }
            1 => {
                let payload = CancelPayload::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::Cancel {
                    lottery_id: payload.lottery_id,
                }
            }
            // added changes
            2 => {
                let payload = CloseVaultPayload::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::CloseVault {
                    lottery_id: payload.lottery_id,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

// helper functions for  deserialization

#[derive(BorshDeserialize, BorshSerialize)]
struct SendSolPayload {
    amount: u64,
    lottery_id: Vec<u8>,
}
#[derive(BorshDeserialize, BorshSerialize)]
struct CancelPayload {
    lottery_id: Vec<u8>,
}
#[derive(BorshDeserialize, BorshSerialize)]
struct CloseVaultPayload {
    lottery_id: Vec<u8>,
}
