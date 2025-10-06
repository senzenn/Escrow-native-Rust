use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use crate::{error::EscrowError, instruction::VaultInstruction, state::LockAccount};

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
                msg!("Instruction: SendSol");
                Self::process_send_sol(program_id, accounts, amount, &lottery_id)
            }
            VaultInstruction::Cancel { lottery_id } => {
                msg!("Instruction: Cancel");
                Self::process_cancel(program_id, accounts, &lottery_id)
            }
            VaultInstruction::CloseVault { lottery_id } => {
                msg!("Instruction: CloseVault");
                Self::process_close_vault(program_id, accounts, &lottery_id)
            }
        }
    }

    /// Process SendSol instruction
    fn process_send_sol(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        lottery_id: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let buyer = next_account_info(account_info_iter)?;
        let vault = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        // Verify buyer is signer
        if !buyer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Derive PDA
        let vault_seed = b"vault";
        let seeds = &[vault_seed.as_ref(), buyer.key.as_ref(), lottery_id];
        let (expected_vault_pda, bump_seed) = Pubkey::find_program_address(seeds, program_id);

        // Verify vault PDA matches
        if expected_vault_pda != *vault.key {
            msg!("Error: PDA mismatch");
            return Err(EscrowError::PDADerivationMismatch.into());
        }

        // Create vault account if it doesn't exist
        if vault.owner != program_id {
            let rent = Rent::get()?;
            let space = LockAccount::LEN;
            let rent_lamports = rent.minimum_balance(space);

            msg!("Creating vault PDA...");
            let create_account_ix = system_instruction::create_account(
                buyer.key,
                vault.key,
                rent_lamports,
                space as u64,
                program_id,
            );

            let signer_seeds: &[&[u8]] = &[
                vault_seed.as_ref(),
                buyer.key.as_ref(),
                lottery_id,
                &[bump_seed],
            ];

            invoke_signed(
                &create_account_ix,
                &[buyer.clone(), vault.clone(), system_program.clone()],
                &[signer_seeds],
            )?;
        }

        // Transfer SOL from buyer to vault
        msg!("Transferring {} lamports to vault", amount);
        let transfer_ix = system_instruction::transfer(buyer.key, vault.key, amount);
        invoke(
            &transfer_ix,
            &[buyer.clone(), vault.clone(), system_program.clone()],
        )?;

        // Write lock account data
        let lock_account = LockAccount {
            buyer: *buyer.key,
            amount,
        };

        let mut vault_data = vault.try_borrow_mut_data()?;
        lock_account.pack_into_slice(&mut vault_data)?;

        msg!("Vault created and funded successfully");
        Ok(())
    }

    /// Process Cancel instruction (refund buyer)
    fn process_cancel(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        lottery_id: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let buyer = next_account_info(account_info_iter)?;
        let vault = next_account_info(account_info_iter)?;

        // Verify buyer is signer
        if !buyer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Verify vault is owned by program
        if vault.owner != program_id {
            return Err(EscrowError::InvalidAccountOwner.into());
        }

        // Derive and verify PDA
        let vault_seed = b"vault";
        let seeds = &[vault_seed.as_ref(), buyer.key.as_ref(), lottery_id];
        let (expected_vault_pda, _bump_seed) = Pubkey::find_program_address(seeds, program_id);

        if expected_vault_pda != *vault.key {
            msg!("Error: PDA mismatch");
            return Err(EscrowError::PDADerivationMismatch.into());
        }

        // Deserialize lock account
        let vault_data = vault.try_borrow_data()?;
        let lock_account = LockAccount::unpack_from_slice(&vault_data)?;

        // Verify buyer matches
        if lock_account.buyer != *buyer.key {
            return Err(EscrowError::InvalidBuyer.into());
        }

        let refund_amount = lock_account.amount;

        // Check vault has enough lamports
        if **vault.lamports.borrow() < refund_amount {
            return Err(EscrowError::InsufficientFunds.into());
        }

        // Transfer lamports from vault back to buyer
        msg!("Refunding {} lamports to buyer", refund_amount);
        **vault.try_borrow_mut_lamports()? -= refund_amount;
        **buyer.try_borrow_mut_lamports()? += refund_amount;

        msg!("Refund completed successfully");
        Ok(())
    }

    /// Process CloseVault instruction (reclaim rent)
    fn process_close_vault(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        lottery_id: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let buyer = next_account_info(account_info_iter)?;
        let vault = next_account_info(account_info_iter)?;

        // Verify buyer is signer
        if !buyer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Verify vault is owned by program
        if vault.owner != program_id {
            return Err(EscrowError::InvalidAccountOwner.into());
        }

        // Derive and verify PDA
        let vault_seed = b"vault";
        let seeds = &[vault_seed.as_ref(), buyer.key.as_ref(), lottery_id];
        let (expected_vault_pda, _bump_seed) = Pubkey::find_program_address(seeds, program_id);

        if expected_vault_pda != *vault.key {
            msg!("Error: PDA mismatch");
            return Err(EscrowError::PDADerivationMismatch.into());
        }

        // Deserialize lock account
        let vault_data = vault.try_borrow_data()?;
        let lock_account = LockAccount::unpack_from_slice(&vault_data)?;

        // Verify buyer matches
        if lock_account.buyer != *buyer.key {
            return Err(EscrowError::InvalidBuyer.into());
        }

        // Close account: transfer all lamports to buyer and zero data
        let vault_lamports = vault.lamports();
        msg!("Closing vault and reclaiming {} lamports", **vault_lamports);

        **vault.try_borrow_mut_lamports()? = 0;
        **buyer.try_borrow_mut_lamports()? = buyer
            .lamports()
            .checked_add(**vault_lamports)
            .ok_or(EscrowError::AmountOverflow)?;

        // Zero out data
        let mut vault_data_mut = vault.try_borrow_mut_data()?;
        vault_data_mut.fill(0);

        msg!("Vault closed successfully");
        Ok(())
    }
}
