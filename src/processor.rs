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
use crate::{error::EscrowError, instructions::EscrowInstruction, state::EscrowAccount};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowInstruction::InitializeEscrow { amount, escrow_id } => {
                msg!("Instruction: InitializeEscrow");
                Self::process_initialize_escrow(program_id, accounts, amount, &escrow_id)
            }
            EscrowInstruction::ReleaseFunds { escrow_id } => {
                msg!("Instruction: ReleaseFunds");
                Self::process_release_funds(program_id, accounts, &escrow_id)
            }
            EscrowInstruction::CancelEscrow { escrow_id } => {
                msg!("Instruction: CancelEscrow");
                Self::process_cancel_escrow(program_id, accounts, &escrow_id)
            }
        }
    }

    /// Process InitializeEscrow instruction
    fn process_initialize_escrow(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        escrow_id: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let buyer = next_account_info(account_info_iter)?;
        let escrow = next_account_info(account_info_iter)?;
        let seller = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        // Verify buyer is signer
        if !buyer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Derive PDA
        let escrow_seed = b"escrow";
        let seeds = &[escrow_seed.as_ref(), buyer.key.as_ref(), escrow_id];
        let (expected_escrow_pda, bump_seed) = Pubkey::find_program_address(seeds, program_id);

        // Verify escrow PDA matches
        if expected_escrow_pda != *escrow.key {
            msg!("Error: PDA mismatch");
            return Err(EscrowError::PDADerivationMismatch.into());
        }

        // Create escrow account if it doesn't exist
        if escrow.owner != program_id {
            let rent = Rent::get()?;
            let space = EscrowAccount::LEN;
            let rent_lamports = rent.minimum_balance(space);

            msg!("Creating escrow PDA...");
            let create_account_ix = system_instruction::create_account(
                buyer.key,
                escrow.key,
                rent_lamports,
                space as u64,
                program_id,
            );

            let signer_seeds: &[&[u8]] = &[
                escrow_seed.as_ref(),
                buyer.key.as_ref(),
                escrow_id,
                &[bump_seed],
            ];

            invoke_signed(
                &create_account_ix,
                &[buyer.clone(), escrow.clone(), system_program.clone()],
                &[signer_seeds],
            )?;
        }

        // Transfer SOL from buyer to escrow
        msg!("Transferring {} lamports to escrow", amount);
        let transfer_ix = system_instruction::transfer(buyer.key, escrow.key, amount);
        invoke(
            &transfer_ix,
            &[buyer.clone(), escrow.clone(), system_program.clone()],
        )?;

        // Write escrow account data
        let escrow_account = EscrowAccount {
            is_initialized: true,
            buyer: *buyer.key,
            seller: *seller.key,
            amount,
        };

        let mut escrow_data = escrow.try_borrow_mut_data()?;
        escrow_account.pack_into_slice(&mut escrow_data)?;

        msg!("Escrow created and funded successfully");
        Ok(())
    }

    /// Process ReleaseFunds instruction (send funds to seller)
    fn process_release_funds(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        escrow_id: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let buyer = next_account_info(account_info_iter)?;
        let escrow = next_account_info(account_info_iter)?;
        let seller = next_account_info(account_info_iter)?;

        // Verify buyer is signer
        if !buyer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Verify escrow is owned by program
        if escrow.owner != program_id {
            return Err(EscrowError::InvalidAccountOwner.into());
        }

        // Derive and verify PDA
        let escrow_seed = b"escrow";
        let seeds = &[escrow_seed.as_ref(), buyer.key.as_ref(), escrow_id];
        let (expected_escrow_pda, _bump_seed) = Pubkey::find_program_address(seeds, program_id);

        if expected_escrow_pda != *escrow.key {
            msg!("Error: PDA mismatch");
            return Err(EscrowError::PDADerivationMismatch.into());
        }

        // Deserialize escrow account
        let escrow_data = escrow.try_borrow_data()?;
        let escrow_account = EscrowAccount::unpack_from_slice(&escrow_data)?;

        // Verify escrow is initialized
        if !escrow_account.is_initialized {
            return Err(EscrowError::NotInitialized.into());
        }

        // Verify buyer matches
        if escrow_account.buyer != *buyer.key {
            return Err(EscrowError::InvalidBuyer.into());
        }

        // Verify seller matches
        if escrow_account.seller != *seller.key {
            return Err(EscrowError::InvalidSeller.into());
        }

        let release_amount = escrow_account.amount;

        // Check escrow has enough lamports
        if **escrow.lamports.borrow() < release_amount {
            return Err(EscrowError::InsufficientFunds.into());
        }

        // Transfer lamports from escrow to seller
        msg!("Releasing {} lamports to seller", release_amount);
        **escrow.try_borrow_mut_lamports()? -= release_amount;
        **seller.try_borrow_mut_lamports()? += release_amount;

        msg!("Funds released to seller successfully");
        Ok(())
    }

    /// Process CancelEscrow instruction (refund buyer)
    fn process_cancel_escrow(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        escrow_id: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let buyer = next_account_info(account_info_iter)?;
        let escrow = next_account_info(account_info_iter)?;

        // Verify buyer is signer
        if !buyer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Verify escrow is owned by program
        if escrow.owner != program_id {
            return Err(EscrowError::InvalidAccountOwner.into());
        }

        // Derive and verify PDA
        let escrow_seed = b"escrow";
        let seeds = &[escrow_seed.as_ref(), buyer.key.as_ref(), escrow_id];
        let (expected_escrow_pda, _bump_seed) = Pubkey::find_program_address(seeds, program_id);

        if expected_escrow_pda != *escrow.key {
            msg!("Error: PDA mismatch");
            return Err(EscrowError::PDADerivationMismatch.into());
        }

        // Deserialize escrow account
        let escrow_data = escrow.try_borrow_data()?;
        let escrow_account = EscrowAccount::unpack_from_slice(&escrow_data)?;

        // Verify escrow is initialized
        if !escrow_account.is_initialized {
            return Err(EscrowError::NotInitialized.into());
        }

        // Verify buyer matches
        if escrow_account.buyer != *buyer.key {
            return Err(EscrowError::InvalidBuyer.into());
        }

        let refund_amount = escrow_account.amount;

        // Check escrow has enough lamports
        if **escrow.lamports.borrow() < refund_amount {
            return Err(EscrowError::InsufficientFunds.into());
        }

        // Transfer lamports from escrow back to buyer
        msg!("Refunding {} lamports to buyer", refund_amount);
        **escrow.try_borrow_mut_lamports()? -= refund_amount;
        **buyer.try_borrow_mut_lamports()? += refund_amount;

        msg!("Refund completed sf8>>>>>");
        Ok(())
    }
}
