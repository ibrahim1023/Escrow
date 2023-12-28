use thiserror::Error;

use solana_program::{address_lookup_table::error, program_error::ProgramError};

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    // Invalid Instruction
    #[error("InvalidInstruction")]
    InvalidInstruction,
    #[error("NotRentExempt")]
    NotRentExempt,
    #[error("ExpectedAmountMismatch")]
    ExpectedAmountMismatch,
    #[error("AmountOverflow")]
    AmountOverflow,
}

impl From<EscrowError> for ProgramError {
    fn from(err: EscrowError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
