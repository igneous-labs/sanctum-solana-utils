//! Functions that interact directly with the runtime such as directly modifying lamports.
//! These functions typically do not require the system_program AccountInfo to be in scope.

use solana_program::{account_info::AccountInfo, program_error::ProgramError, system_program};

use crate::{onchain_rent_exempt_lamports_for, TransferAccounts};

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

pub trait ResizableAccount {
    /// `realloc()`s an account without zeroing data, returning the rent-exempt minimum balance for its new length
    fn realloc_return_new_rent_exempt_balance(&self, new_len: usize) -> Result<u64, ProgramError>;

    fn curr_lamports(&self) -> u64;

    fn curr_data_len(&self) -> usize;

    /// Extend this account to `new_len` with `realloc()`, returning the additional lamports that needs to be transferred in
    /// given its new rent-exempt balance requirements
    fn extend_to(&self, new_len: usize) -> Result<u64, ProgramError> {
        let new_rent_exempt_min = self.realloc_return_new_rent_exempt_balance(new_len)?;
        Ok(new_rent_exempt_min.saturating_sub(self.curr_lamports()))
    }

    /// Extend this account by `inc` bytes with `realloc()`, returning the additional lamports that needs to be transferred in
    /// given its new rent-exempt balance requirements
    fn extend_by(&self, inc: usize) -> Result<u64, ProgramError> {
        let new_len = self
            .curr_data_len()
            .checked_add(inc)
            .ok_or(ProgramError::InvalidAccountData)?;
        self.extend_to(new_len)
    }

    /// Shrinks this account to `new_len` with `realloc()`, returning the excess lamports that can be transferred out
    /// given its new rent-exempt balance requirements
    fn shrink_to(&self, new_len: usize) -> Result<u64, ProgramError> {
        let new_rent_exempt_min = self.realloc_return_new_rent_exempt_balance(new_len)?;
        Ok(self.curr_lamports().saturating_sub(new_rent_exempt_min))
    }

    /// Shrinks this account by `dec` bytes with `realloc()`, returning the excess lamports that can be transferred out
    /// given its new rent-exempt balance requirements
    fn shrink_by(&self, dec: usize) -> Result<u64, ProgramError> {
        let new_len = self
            .curr_data_len()
            .checked_sub(dec)
            .ok_or(ProgramError::InvalidAccountData)?;
        self.shrink_to(new_len)
    }
}

impl ResizableAccount for AccountInfo<'_> {
    fn realloc_return_new_rent_exempt_balance(&self, new_len: usize) -> Result<u64, ProgramError> {
        self.realloc(new_len, false)?;
        onchain_rent_exempt_lamports_for(new_len)
    }

    fn curr_lamports(&self) -> u64 {
        self.lamports()
    }

    fn curr_data_len(&self) -> usize {
        self.data_len()
    }
}

impl<T: ResizableAccount + ?Sized> ResizableAccount for &T {
    fn realloc_return_new_rent_exempt_balance(&self, new_len: usize) -> Result<u64, ProgramError> {
        (*self).realloc_return_new_rent_exempt_balance(new_len)
    }

    fn curr_lamports(&self) -> u64 {
        (*self).curr_lamports()
    }

    fn curr_data_len(&self) -> usize {
        (*self).curr_data_len()
    }
}
