use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::{FreezeAccountKeys, SplTokenError, ThawAccountKeys};

use crate::{ReadonlyMintAccount, ReadonlyTokenAccount};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreezeThawFreeAccounts<A, M> {
    pub token_account: A,
    pub mint: M,
}

impl<
        A: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountData + ReadonlyAccountPubkey,
    > FreezeThawFreeAccounts<A, M>
{
    pub fn resolve_freeze(self) -> Result<FreezeAccountKeys, ProgramError> {
        self.check_token_account()?;
        let intermediate: FreezeThawAccountsUncheckedTokenAccount<M> = self.into();
        intermediate.resolve_freeze()
    }

    pub fn resolve_thaw(self) -> Result<ThawAccountKeys, ProgramError> {
        self.check_token_account()?;
        let intermediate: FreezeThawAccountsUncheckedTokenAccount<M> = self.into();
        intermediate.resolve_thaw()
    }

    fn check_token_account(&self) -> Result<(), ProgramError> {
        let t = ReadonlyTokenAccount(&self.token_account)
            .try_into_valid()?
            .try_into_initialized()?;
        if t.token_account_mint() != *self.mint.pubkey() {
            return Err(SplTokenError::MintMismatch.into());
        }
        Ok(())
    }
}

impl<
        A: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountData + ReadonlyAccountPubkey,
    > TryFrom<FreezeThawFreeAccounts<A, M>> for FreezeAccountKeys
{
    type Error = ProgramError;

    fn try_from(value: FreezeThawFreeAccounts<A, M>) -> Result<Self, Self::Error> {
        value.resolve_freeze()
    }
}

impl<
        A: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountData + ReadonlyAccountPubkey,
    > TryFrom<FreezeThawFreeAccounts<A, M>> for ThawAccountKeys
{
    type Error = ProgramError;

    fn try_from(value: FreezeThawFreeAccounts<A, M>) -> Result<Self, Self::Error> {
        value.resolve_thaw()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreezeThawAccountsUncheckedTokenAccount<M> {
    pub token_account: Pubkey,
    pub mint: M,
}

impl<M: ReadonlyAccountData + ReadonlyAccountPubkey> FreezeThawAccountsUncheckedTokenAccount<M> {
    pub fn resolve_freeze(&self) -> Result<FreezeAccountKeys, ProgramError> {
        let authority = self.mint_freeze_authority()?;
        Ok(FreezeAccountKeys {
            mint: *self.mint.pubkey(),
            token_account: self.token_account,
            authority,
        })
    }

    pub fn resolve_thaw(&self) -> Result<ThawAccountKeys, ProgramError> {
        let authority = self.mint_freeze_authority()?;
        Ok(ThawAccountKeys {
            mint: *self.mint.pubkey(),
            token_account: self.token_account,
            authority,
        })
    }

    fn mint_freeze_authority(&self) -> Result<Pubkey, ProgramError> {
        let m = ReadonlyMintAccount(&self.mint)
            .try_into_valid()?
            .try_into_initialized()?;
        Ok(m.mint_freeze_authority()
            .ok_or(SplTokenError::MintCannotFreeze)?)
    }
}

impl<A: ReadonlyAccountPubkey, M: ReadonlyAccountData + ReadonlyAccountPubkey>
    From<FreezeThawFreeAccounts<A, M>> for FreezeThawAccountsUncheckedTokenAccount<M>
{
    fn from(
        FreezeThawFreeAccounts {
            token_account,
            mint,
        }: FreezeThawFreeAccounts<A, M>,
    ) -> Self {
        Self {
            token_account: *token_account.pubkey(),
            mint,
        }
    }
}

impl<M: ReadonlyAccountData + ReadonlyAccountPubkey>
    TryFrom<FreezeThawAccountsUncheckedTokenAccount<M>> for FreezeAccountKeys
{
    type Error = ProgramError;

    fn try_from(value: FreezeThawAccountsUncheckedTokenAccount<M>) -> Result<Self, Self::Error> {
        value.resolve_freeze()
    }
}

impl<M: ReadonlyAccountData + ReadonlyAccountPubkey>
    TryFrom<FreezeThawAccountsUncheckedTokenAccount<M>> for ThawAccountKeys
{
    type Error = ProgramError;

    fn try_from(value: FreezeThawAccountsUncheckedTokenAccount<M>) -> Result<Self, Self::Error> {
        value.resolve_thaw()
    }
}
