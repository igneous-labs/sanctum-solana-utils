use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::TransferCheckedKeys;

use crate::ReadonlyTokenAccount;

#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedFreeArgs<F: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub from: F,
    pub to: Pubkey,
}

impl<F: ReadonlyAccountPubkey + ReadonlyAccountData> TransferCheckedFreeArgs<F> {
    pub fn resolve(&self) -> Result<TransferCheckedKeys, ProgramError> {
        let Self { from, to } = self;

        if !from.token_account_data_is_valid() || !from.token_account_is_initialized() {
            return Err(ProgramError::InvalidAccountData);
        }

        let mint = from.token_account_mint();
        let authority = from.token_account_authority();

        Ok(TransferCheckedKeys {
            from: *from.pubkey(),
            mint,
            to: *to,
            authority,
        })
    }
}
