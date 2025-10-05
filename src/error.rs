use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Notr rent exampt")]
    NotRentExempt,
    #[error("Expected amount mismatch ")]
    ExpectedAmountMismatch,
    #[error("Amount overflow ")]
    AmountOverFlow,
    #[error("PDA Derivation mismatch")]
    PDADerivationMismatch,
    #[error("InsufficientFunds")]
    InsufficientFunds,
    #[error("Account not owned by program")]
    InvalildAccountOwner,
}

// implementint the custom error for solana program
impl From<EscrowError> for ProgramError {
    // this  wiil directlyy match the custom errors with some of the predefined Solana Structs

    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
