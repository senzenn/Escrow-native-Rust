use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Expected amount mismatch")]
    ExpectedAmountMismatch,
    #[error("Amount overflow")]
    AmountOverflow,
    #[error("PDA Derivation mismatch")]
    PDADerivationMismatch,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Account not owned by program")]
    InvalidAccountOwner,
    #[error("Invalid buyer")]
    InvalidBuyer,
    #[error("Invalid seller")]
    InvalidSeller,
    #[error("Escrow already initialized")]
    AlreadyInitialized,
    #[error("Escrow not initialized")]
    NotInitialized,
    #[error("Invalid milestone")]
    InvalidMilestone,
}

// implementint the custom error for solana program
impl From<EscrowError> for ProgramError {
    // this  wiil directlyy match the custom errors with some of the predefined Solana Structs

    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
