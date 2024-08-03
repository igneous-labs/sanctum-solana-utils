use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_token_interface::TransferCheckedKeys;

use crate::ReadonlyTokenAccount;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransferCheckedFreeAccounts<A> {
    pub from: A,
    pub to: Pubkey,
}

impl<A: ReadonlyAccountPubkeyBytes + ReadonlyAccountData> TransferCheckedFreeAccounts<A> {
    pub fn resolve(&self) -> Result<TransferCheckedKeys, ProgramError> {
        let Self { from, to } = self;

        let f = ReadonlyTokenAccount(&self.from)
            .try_into_valid()?
            .try_into_initialized()?;

        let mint = f.token_account_mint();
        let authority = f.token_account_authority();

        Ok(TransferCheckedKeys {
            from: Pubkey::new_from_array(from.pubkey_bytes()),
            mint,
            to: *to,
            authority,
        })
    }
}

impl<A: ReadonlyAccountPubkeyBytes + ReadonlyAccountData> TryFrom<TransferCheckedFreeAccounts<A>>
    for TransferCheckedKeys
{
    type Error = ProgramError;

    fn try_from(value: TransferCheckedFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
