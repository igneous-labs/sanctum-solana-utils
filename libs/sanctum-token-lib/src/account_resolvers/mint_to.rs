use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::{MintToCheckedKeys, MintToKeys, SplTokenError};

use crate::{ReadonlyMintAccount, ReadonlyTokenAccount};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MintToFreeAccounts<A, M> {
    pub token_account: A,
    pub mint: M,
}

impl<
        A: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountData + ReadonlyAccountPubkey,
    > MintToFreeAccounts<A, M>
{
    pub fn resolve(self) -> Result<MintToKeys, ProgramError> {
        self.check_token_account()?;
        let intermediate: MintToAccountsUncheckedTokenAccount<M> = self.into();
        intermediate.resolve()
    }

    pub fn resolve_checked(self) -> Result<MintToCheckedKeys, ProgramError> {
        self.check_token_account()?;
        let intermediate: MintToAccountsUncheckedTokenAccount<M> = self.into();
        intermediate.resolve_checked()
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
    > TryFrom<MintToFreeAccounts<A, M>> for MintToKeys
{
    type Error = ProgramError;

    fn try_from(value: MintToFreeAccounts<A, M>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

impl<
        A: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountData + ReadonlyAccountPubkey,
    > TryFrom<MintToFreeAccounts<A, M>> for MintToCheckedKeys
{
    type Error = ProgramError;

    fn try_from(value: MintToFreeAccounts<A, M>) -> Result<Self, Self::Error> {
        value.resolve_checked()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MintToAccountsUncheckedTokenAccount<M> {
    pub token_account: Pubkey,
    pub mint: M,
}

impl<M: ReadonlyAccountData + ReadonlyAccountPubkey> MintToAccountsUncheckedTokenAccount<M> {
    pub fn resolve(&self) -> Result<MintToKeys, ProgramError> {
        let authority = self.mint_authority()?;
        Ok(MintToKeys {
            mint: *self.mint.pubkey(),
            token_account: self.token_account,
            authority,
        })
    }

    pub fn resolve_checked(&self) -> Result<MintToCheckedKeys, ProgramError> {
        let authority = self.mint_authority()?;
        Ok(MintToCheckedKeys {
            mint: *self.mint.pubkey(),
            token_account: self.token_account,
            authority,
        })
    }

    fn mint_authority(&self) -> Result<Pubkey, ProgramError> {
        let m = ReadonlyMintAccount(&self.mint)
            .try_into_valid()?
            .try_into_initialized()?;
        Ok(m.mint_mint_authority().ok_or(SplTokenError::FixedSupply)?)
    }
}

impl<A: ReadonlyAccountPubkey, M: ReadonlyAccountData + ReadonlyAccountPubkey>
    From<MintToFreeAccounts<A, M>> for MintToAccountsUncheckedTokenAccount<M>
{
    fn from(
        MintToFreeAccounts {
            token_account,
            mint,
        }: MintToFreeAccounts<A, M>,
    ) -> Self {
        Self {
            token_account: *token_account.pubkey(),
            mint,
        }
    }
}

impl<M: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<MintToAccountsUncheckedTokenAccount<M>>
    for MintToKeys
{
    type Error = ProgramError;

    fn try_from(value: MintToAccountsUncheckedTokenAccount<M>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

impl<M: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<MintToAccountsUncheckedTokenAccount<M>>
    for MintToCheckedKeys
{
    type Error = ProgramError;

    fn try_from(value: MintToAccountsUncheckedTokenAccount<M>) -> Result<Self, Self::Error> {
        value.resolve_checked()
    }
}
