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
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = VaultInstruction::unpack(instruction_data)?;
        match instruction {
            VaultInstruction::SendSol { amount, lottery_id } => {
                msg!("Instruction Send Sol ");
                //TODO: main logic
            }
            VaultInstruction::Cancel { lottery_id } => {
                msg!("Instruction: Cancel");
                //TODO: main logic
            }
            VaultInstruction::CloseVault { lottery_id } => {
                msg!("Instuction Close");
                //TODO: main logic
            }
        }
    }

    // process send sol function defination

    fn process_send_sol(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        lottery_id: &[u8],
    ) {
        let account_info_iter = &mut accounts.iter();
        let buyer = next_account_info(account_info_iter)?;
        let vault = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        // validate buyer is signer important validator
        if !buyer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // PDA

        let vault_seed = b"vault";
        let seed = &[vault_seed.as_ref(), buyer.key.as_ref(), lottery_id];
    }
}
