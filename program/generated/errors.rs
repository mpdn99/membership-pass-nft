// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Error, FromPrimitive, Debug, Clone)]
pub enum NftMembershipError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid Signer Permission")]
    InvalidSignerPermission,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,

    #[error("Executable Account Expected")]
    ExecutableAccountExpected,

 
}

impl From<NftMembershipError> for ProgramError {
    fn from(e: NftMembershipError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for NftMembershipError {
    fn type_of() -> &'static str {
        "NftMembershipError"
    }
}

impl PrintProgramError for NftMembershipError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            NftMembershipError::InvalidInstruction => msg!("Error: Invalid instruction"),
            NftMembershipError::InvalidSignerPermission => msg!("Error: The account is_signer value is not the expected one"),
            NftMembershipError::NotExpectedAddress => {
                msg!("Error: Not the expected account address")
            }
            NftMembershipError::WrongAccountOwner => msg!("Error: Wrong account owner"),
            NftMembershipError::InvalidAccountLen => msg!("Error: Invalid account length"),
            NftMembershipError::ExecutableAccountExpected => msg!("Error: Executable account expected"),
 
        }
    }
}