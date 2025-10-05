#![cfg_attr(not(feature = "no-entrypoint"), no_std)]

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::processor::Processor;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, _instruction_data);

    Ok(())
}
