use std::num::NonZeroU32;

use solana_program::{program_error::ProgramError, pubkey::Pubkey, stake, system_program, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_stake_pool_interface::{AddValidatorToPoolKeys, StakePool};

use crate::{
    deserialize_stake_pool_checked, FindValidatorStakeAccount, FindValidatorStakeAccountArgs,
    FindWithdrawAuthority,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddValidatorToPool<P> {
    pub stake_pool: P,
    pub vote_account: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddValidatorToPoolPdas {
    pub withdraw_authority: Pubkey,
    pub validator_stake_account: Pubkey,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> AddValidatorToPool<P> {
    pub fn resolve_with_pdas(
        &self,
        AddValidatorToPoolPdas {
            withdraw_authority,
            validator_stake_account,
        }: AddValidatorToPoolPdas,
    ) -> Result<AddValidatorToPoolKeys, ProgramError> {
        let StakePool {
            staker,
            validator_list,
            reserve_stake,
            ..
        } = deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(AddValidatorToPoolKeys {
            stake_pool: Pubkey::new_from_array(self.stake_pool.pubkey_bytes()),
            staker,
            reserve_stake,
            withdraw_authority,
            validator_list,
            validator_stake_account,
            vote_account: self.vote_account,
            rent: sysvar::rent::ID,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
            stake_config: stake::config::ID,
            system_program: system_program::ID,
            stake_program: stake::program::ID,
        })
    }

    pub fn resolve_for_prog(
        &self,
        program_id: &Pubkey,
        seed: Option<NonZeroU32>,
    ) -> Result<AddValidatorToPoolKeys, ProgramError> {
        let (withdraw_authority, _bump) = FindWithdrawAuthority {
            pool: Pubkey::new_from_array(self.stake_pool.pubkey_bytes()),
        }
        .run_for_prog(program_id);
        let (validator_stake_account, _bump) =
            FindValidatorStakeAccount::new(FindValidatorStakeAccountArgs {
                pool: Pubkey::new_from_array(self.stake_pool.pubkey_bytes()),
                vote: self.vote_account,
                seed,
            })
            .run_for_prog(program_id);
        self.resolve_with_pdas(AddValidatorToPoolPdas {
            withdraw_authority,
            validator_stake_account,
        })
    }
}
