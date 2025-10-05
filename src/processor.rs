#![cfg_attr(not(feature = "no-entrypoint"), no_std)]

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    rent::Rent,
    system_instruction,
    program_error::ProgramError,
    sysvar::Sysvar,
    pubkey::Pubkey,
};

use crate::{error::EscrowError, instructions::VaultInstruction, state::LockAccount};

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, account_info: &AccountInfo) -> ProgramResult {
        Ok(())
    }
}

//pub fn process_instruction(
//    _program_id: &Pubkey,
//    accounts: &[AccountInfo],
//    _instruction_data: &[u8],
//) -> ProgramResult {
//    Ok(())
//}
