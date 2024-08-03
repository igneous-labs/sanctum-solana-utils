use solana_program::{program_error::ProgramError, pubkey::Pubkey, stake};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use stake_program_interface::RedelegateKeys;

use crate::ReadonlyStakeAccount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RedelegateFreeAccounts<S> {
    pub stake: S,
    pub uninitialized_stake: Pubkey,
    pub vote: Pubkey,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> RedelegateFreeAccounts<S> {
    pub fn resolve_to_free_keys(&self) -> Result<RedelegateFreeKeys, ProgramError> {
        let Self {
            stake,
            uninitialized_stake,
            vote,
        } = self;
        let s = ReadonlyStakeAccount(stake);
        let s = s.try_into_valid()?;
        let s = s.try_into_stake_or_initialized()?;
        Ok(RedelegateFreeKeys {
            stake: Pubkey::new_from_array(stake.pubkey_bytes()),
            uninitialized_stake: *uninitialized_stake,
            vote: *vote,
            stake_authority: s.stake_meta_authorized_staker(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RedelegateFreeKeys {
    pub stake: Pubkey,
    pub uninitialized_stake: Pubkey,
    pub vote: Pubkey,
    pub stake_authority: Pubkey,
}

impl RedelegateFreeKeys {
    pub fn resolve(&self) -> RedelegateKeys {
        let Self {
            stake,
            uninitialized_stake,
            vote,
            stake_authority,
        } = self;
        RedelegateKeys {
            stake: *stake,
            uninitialized_stake: *uninitialized_stake,
            vote: *vote,
            stake_authority: *stake_authority,
            stake_config: stake::config::ID,
        }
    }
}

impl From<RedelegateFreeKeys> for RedelegateKeys {
    fn from(value: RedelegateFreeKeys) -> Self {
        value.resolve()
    }
}
