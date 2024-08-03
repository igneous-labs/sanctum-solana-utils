use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_token_interface::{FreezeAccountKeys, SplTokenError, ThawAccountKeys};

use crate::{ReadonlyMintAccount, ReadonlyTokenAccount};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreezeThawFreeAccounts<A, M> {
    pub token_account: A,
    pub mint: M,
}

impl<
        A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes,
        M: ReadonlyAccountData + ReadonlyAccountPubkeyBytes,
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
        if t.token_account_mint() != Pubkey::new_from_array(self.mint.pubkey_bytes()) {
            return Err(SplTokenError::MintMismatch.into());
        }
        Ok(())
    }
}

impl<
        A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes,
        M: ReadonlyAccountData + ReadonlyAccountPubkeyBytes,
    > TryFrom<FreezeThawFreeAccounts<A, M>> for FreezeAccountKeys
{
    type Error = ProgramError;

    fn try_from(value: FreezeThawFreeAccounts<A, M>) -> Result<Self, Self::Error> {
        value.resolve_freeze()
    }
}

impl<
        A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes,
        M: ReadonlyAccountData + ReadonlyAccountPubkeyBytes,
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

impl<M: ReadonlyAccountData + ReadonlyAccountPubkeyBytes>
    FreezeThawAccountsUncheckedTokenAccount<M>
{
    pub fn resolve_freeze(&self) -> Result<FreezeAccountKeys, ProgramError> {
        let authority = self.mint_freeze_authority()?;
        Ok(FreezeAccountKeys {
            mint: Pubkey::new_from_array(self.mint.pubkey_bytes()),
            token_account: self.token_account,
            authority,
        })
    }

    pub fn resolve_thaw(&self) -> Result<ThawAccountKeys, ProgramError> {
        let authority = self.mint_freeze_authority()?;
        Ok(ThawAccountKeys {
            mint: Pubkey::new_from_array(self.mint.pubkey_bytes()),
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

impl<A: ReadonlyAccountPubkeyBytes, M: ReadonlyAccountData + ReadonlyAccountPubkeyBytes>
    From<FreezeThawFreeAccounts<A, M>> for FreezeThawAccountsUncheckedTokenAccount<M>
{
    fn from(
        FreezeThawFreeAccounts {
            token_account,
            mint,
        }: FreezeThawFreeAccounts<A, M>,
    ) -> Self {
        Self {
            token_account: Pubkey::new_from_array(token_account.pubkey_bytes()),
            mint,
        }
    }
}

impl<M: ReadonlyAccountData + ReadonlyAccountPubkeyBytes>
    TryFrom<FreezeThawAccountsUncheckedTokenAccount<M>> for FreezeAccountKeys
{
    type Error = ProgramError;

    fn try_from(value: FreezeThawAccountsUncheckedTokenAccount<M>) -> Result<Self, Self::Error> {
        value.resolve_freeze()
    }
}

impl<M: ReadonlyAccountData + ReadonlyAccountPubkeyBytes>
    TryFrom<FreezeThawAccountsUncheckedTokenAccount<M>> for ThawAccountKeys
{
    type Error = ProgramError;

    fn try_from(value: FreezeThawAccountsUncheckedTokenAccount<M>) -> Result<Self, Self::Error> {
        value.resolve_thaw()
    }
}
