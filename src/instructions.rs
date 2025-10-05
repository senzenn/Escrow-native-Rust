use std::u8;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum VaultInstruct {
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
