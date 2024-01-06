use solana_program::program_error::ProgramError;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::{AuthorityType, SetAuthorityKeys, SplTokenError};

use crate::{ReadonlyMintAccount, ReadonlyTokenAccount};

#[derive(Clone, Debug, PartialEq)]
pub struct SetAuthorityFreeArgs<A> {
    pub account: A,
    pub authority_type: AuthorityType,
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> SetAuthorityFreeArgs<A> {
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
        let m = ReadonlyMintAccount(&self.account)
            .try_into_valid()?
            .try_into_initialized()?;
        let authority = match authority_type {
            AuthorityType::FreezeAccount => m
                .mint_freeze_authority()
                .ok_or(SplTokenError::MintCannotFreeze)?,
            AuthorityType::MintTokens => {
                m.mint_mint_authority().ok_or(SplTokenError::FixedSupply)?
            }
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
        let t = ReadonlyTokenAccount(&self.account)
            .try_into_valid()?
            .try_into_initialized()?;
        let token_auth = t.token_account_authority();
        let authority = match authority_type {
            AuthorityType::AccountOwner => token_auth,
            AuthorityType::CloseAccount => t.token_account_close_authority().unwrap_or(token_auth),
            _ => unreachable!(),
        };
        Ok(SetAuthorityKeys {
            authority,
            account: *account.pubkey(),
        })
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<SetAuthorityFreeArgs<A>>
    for SetAuthorityKeys
{
    type Error = ProgramError;

    fn try_from(value: SetAuthorityFreeArgs<A>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
