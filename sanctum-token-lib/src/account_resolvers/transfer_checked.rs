use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};
use spl_token_2022::extension::StateWithExtensions;

use crate::TransferCheckedKeys;

#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedFreeArgs<
    F: ReadonlyAccountPubkey + ReadonlyAccountOwner + ReadonlyAccountData,
> {
    pub from: F,
    pub to: Pubkey,
}

impl<F: ReadonlyAccountPubkey + ReadonlyAccountOwner + ReadonlyAccountData>
    TransferCheckedFreeArgs<F>
{
    pub fn resolve(&self) -> Result<TransferCheckedKeys, ProgramError> {
        let Self { from, to } = self;
        let data = from.data();
        let state = StateWithExtensions::<spl_token_2022::state::Account>::unpack(&data)?;
        let mint = state.base.mint;
        let authority = state.base.owner;
        Ok(TransferCheckedKeys {
            token_program: *from.owner(),
            from: *from.pubkey(),
            mint,
            to: *to,
            authority,
        })
    }
}
