// entrypoint

pub mod entrypoint;
pub mod instructions;
pub mod processor;
pub mod state;
pub mod error;

pub use solana_program;

use crate::entrypoint::process_instruction;
solana_program::entrypoint!(process_instruction);
