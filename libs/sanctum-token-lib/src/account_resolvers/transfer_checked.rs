use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::TransferCheckedKeys;

use crate::ReadonlyTokenAccount;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransferCheckedFreeAccounts<A> {
    pub from: A,
    pub to: Pubkey,
}

impl<A: ReadonlyAccountPubkey + ReadonlyAccountData> TransferCheckedFreeAccounts<A> {
    pub fn resolve(&self) -> Result<TransferCheckedKeys, ProgramError> {
        let Self { from, to } = self;

        let f = ReadonlyTokenAccount(&self.from)
            .try_into_valid()?
            .try_into_initialized()?;

        let mint = f.token_account_mint();
        let authority = f.token_account_authority();

        Ok(TransferCheckedKeys {
            from: *from.pubkey(),
            mint,
            to: *to,
            authority,
        })
    }
}

impl<A: ReadonlyAccountPubkey + ReadonlyAccountData> TryFrom<TransferCheckedFreeAccounts<A>>
    for TransferCheckedKeys
{
    type Error = ProgramError;

    fn try_from(value: TransferCheckedFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
