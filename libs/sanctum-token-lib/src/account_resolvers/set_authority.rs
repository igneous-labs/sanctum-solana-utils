use solana_program::program_error::ProgramError;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::{AuthorityType, SetAuthorityKeys, SplTokenError};

use crate::{ReadonlyMintAccount, ReadonlyTokenAccount};

#[derive(Clone, Debug, PartialEq)]
pub struct SetAuthorityFreeArgs<D> {
    pub account: D,
    pub authority_type: AuthorityType,
}

impl<D: ReadonlyAccountData + ReadonlyAccountPubkey> SetAuthorityFreeArgs<D> {
    pub fn resolve(&self) -> Result<SetAuthorityKeys, ProgramError> {
        match self.authority_type {
            AuthorityType::FreezeAccount | AuthorityType::MintTokens => self.resolve_mint(),
            AuthorityType::AccountOwner | AuthorityType::CloseAccount => {
                self.resolve_token_account()
            }
        }
    }

    fn resolve_mint(&self) -> Result<SetAuthorityKeys, ProgramError> {
        let Self {
            account,
            authority_type,
        } = self;
        if !account.mint_data_is_valid() || !account.mint_is_initialized() {
            return Err(ProgramError::InvalidAccountData);
        }
        let authority = match authority_type {
            AuthorityType::FreezeAccount => account
                .mint_freeze_authority()
                .ok_or(SplTokenError::MintCannotFreeze)?,
            AuthorityType::MintTokens => account
                .mint_mint_authority()
                .ok_or(SplTokenError::FixedSupply)?,
            _ => unreachable!(),
        };
        Ok(SetAuthorityKeys {
            authority,
            account: *account.pubkey(),
        })
    }

    fn resolve_token_account(&self) -> Result<SetAuthorityKeys, ProgramError> {
        let Self {
            account,
            authority_type,
        } = self;
        if !account.token_account_data_is_valid() || !account.token_account_is_initialized() {
            return Err(ProgramError::InvalidAccountData);
        }
        let token_auth = account.token_account_authority();
        let authority = match authority_type {
            AuthorityType::AccountOwner => token_auth,
            AuthorityType::CloseAccount => account
                .token_account_close_authority()
                .unwrap_or(token_auth),
            _ => unreachable!(),
        };
        Ok(SetAuthorityKeys {
            authority,
            account: *account.pubkey(),
        })
    }
}

impl<D: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<SetAuthorityFreeArgs<D>>
    for SetAuthorityKeys
{
    type Error = ProgramError;

    fn try_from(value: SetAuthorityFreeArgs<D>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
