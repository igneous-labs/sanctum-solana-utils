use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_stake_pool_interface::{SetFundingAuthorityKeys, StakePool};

use crate::deserialize_stake_pool_checked;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetFundingAuthority<P> {
    pub stake_pool: P,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> SetFundingAuthority<P> {
    pub fn resolve_with_new_auth(
        &self,
        new_funding_authority: Pubkey,
    ) -> Result<SetFundingAuthorityKeys, ProgramError> {
        let StakePool { manager, .. } =
            deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(SetFundingAuthorityKeys {
            stake_pool: Pubkey::new_from_array(self.stake_pool.pubkey_bytes()),
            manager,
            new_funding_authority,
        })
    }

    pub fn resolve_with_auth_removed(&self) -> Result<[AccountMeta; 2], ProgramError> {
        let [pool, manager, _ignored] = self.resolve_with_new_auth(Pubkey::default())?.into();
        Ok([pool, manager])
    }
}
