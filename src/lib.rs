// entrypoint

pub mod entrypoint;
pub mod instructions;
pub mod processor;
pub mod state;
pub mod error;

pub use solana_program;

solana_program::entrypoint!(entrypoint::process_instruction);
