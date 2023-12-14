//! Functions that interact directly with the runtime such as directly modifying lamports.
//! These functions typically do not require the system_program AccountInfo to be in scope.

use solana_program::{account_info::AccountInfo, program_error::ProgramError, system_program};

use crate::TransferAccounts;

/// Transfer by directly decrementing one account's lamports and
/// incrementing another's
pub fn transfer_direct_increment(
    TransferAccounts { from, to }: TransferAccounts,
    lamports: u64,
) -> Result<(), ProgramError> {
    let to_starting_lamports = to.lamports();
    let from_starting_lamports = from.lamports();
    **to.try_borrow_mut_lamports()? = to_starting_lamports
        .checked_add(lamports)
        .ok_or(ProgramError::InvalidArgument)?;
    **from.try_borrow_mut_lamports()? = from_starting_lamports
        .checked_sub(lamports)
        .ok_or(ProgramError::InvalidArgument)?;
    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub struct CloseAccountAccounts<'me, 'info> {
    pub refund_rent_to: &'me AccountInfo<'info>,
    pub close: &'me AccountInfo<'info>,
}

pub fn close_account(
    CloseAccountAccounts {
        refund_rent_to,
        close,
    }: CloseAccountAccounts,
) -> Result<(), ProgramError> {
    transfer_direct_increment(
        TransferAccounts {
            from: close,
            to: refund_rent_to,
        },
        close.lamports(),
    )?;
    close.assign(&system_program::ID);
    close.realloc(0, false)
}
