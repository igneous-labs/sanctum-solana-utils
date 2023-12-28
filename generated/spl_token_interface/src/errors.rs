use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum SplTokenError {
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt = 0,
    #[error("Insufficient funds")]
    InsufficientFunds = 1,
    #[error("Invalid Mint")]
    InvalidMint = 2,
    #[error("Account not associated with this Mint")]
    MintMismatch = 3,
    #[error("Owner does not match")]
    OwnerMismatch = 4,
    #[error("Fixed supply")]
    FixedSupply = 5,
    #[error("Already in use")]
    AlreadyInUse = 6,
    #[error("Invalid number of provided signers")]
    InvalidNumberOfProvidedSigners = 7,
    #[error("Invalid number of required signers")]
    InvalidNumberOfRequiredSigners = 8,
    #[error("State is uninitialized")]
    UninitializedState = 9,
    #[error("Instruction does not support native tokens")]
    NativeNotSupported = 10,
    #[error("Non-native account can only be closed if its balance is zero")]
    NonNativeHasBalance = 11,
    #[error("Invalid instruction")]
    InvalidInstruction = 12,
    #[error("State is invalid for requested operation")]
    InvalidState = 13,
    #[error("Operation overflowed")]
    Overflow = 14,
    #[error("Account does not support specified authority type")]
    AuthorityTypeNotSupported = 15,
    #[error("This token mint cannot freeze accounts")]
    MintCannotFreeze = 16,
    #[error("Account is frozen")]
    AccountFrozen = 17,
    #[error("The provided decimals value different from the Mint decimals")]
    MintDecimalsMismatch = 18,
    #[error("Instruction does not support non-native tokens")]
    NonNativeNotSupported = 19,
}
impl From<SplTokenError> for ProgramError {
    fn from(e: SplTokenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for SplTokenError {
    fn type_of() -> &'static str {
        "SplTokenError"
    }
}
impl PrintProgramError for SplTokenError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
