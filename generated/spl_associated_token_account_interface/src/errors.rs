use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum SplAssociatedTokenAccountError {
    #[error("Associated token account owner does not match address derivation")]
    InvalidOwner = 0,
}
impl From<SplAssociatedTokenAccountError> for ProgramError {
    fn from(e: SplAssociatedTokenAccountError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for SplAssociatedTokenAccountError {
    fn type_of() -> &'static str {
        "SplAssociatedTokenAccountError"
    }
}
impl PrintProgramError for SplAssociatedTokenAccountError {
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
