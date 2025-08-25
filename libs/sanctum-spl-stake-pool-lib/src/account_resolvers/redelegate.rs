use std::num::NonZeroU32;

use solana_program::{pubkey::Pubkey, stake, system_program, sysvar};
use solana_readonly_account::keyed::Keyed;
use spl_stake_pool_interface::{RedelegateKeys, StakePool};

use crate::{
    FindEphemeralStakeAccount, FindEphemeralStakeAccountArgs, FindTransientStakeAccount,
    FindTransientStakeAccountArgs, FindValidatorStakeAccount, FindValidatorStakeAccountArgs,
    FindWithdrawAuthority,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RedelegateValidatorStakeSeeds {
    pub src_validator: Option<NonZeroU32>,
    pub dst_validator: Option<NonZeroU32>,
    pub src_transient: u64,
    pub dst_transient: u64,
    pub ephemeral: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RedelegatePdas {
    pub withdraw_authority: Pubkey,
    pub src_validator_stake_account: Pubkey,
    pub src_transient_stake_account: Pubkey,
    pub dst_validator_stake_account: Pubkey,
    pub dst_transient_stake_account: Pubkey,
    pub ephemeral_stake_account: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RedelegateVoteAccounts {
    pub src: Pubkey,
    pub dst: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Redelegate<'a> {
    pub pool: Keyed<&'a StakePool>,
}

impl<'a> Redelegate<'a> {
    pub fn resolve_with_pdas(
        &self,
        dst_vote_account: Pubkey,
        RedelegatePdas {
            withdraw_authority,
            src_validator_stake_account,
            src_transient_stake_account,
            dst_validator_stake_account,
            dst_transient_stake_account,
            ephemeral_stake_account,
        }: RedelegatePdas,
    ) -> RedelegateKeys {
        let Keyed {
            pubkey: stake_pool,
            account:
                StakePool {
                    staker,
                    validator_list,
                    reserve_stake,
                    ..
                },
        } = self.pool;
        RedelegateKeys {
            stake_pool,
            staker: *staker,
            withdraw_authority,
            validator_list: *validator_list,
            reserve_stake: *reserve_stake,
            src_validator_stake_account,
            src_transient_stake_account,
            ephemeral_stake_account,
            dst_transient_stake_account,
            dst_validator_stake_account,
            dst_vote_account,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
            stake_config: stake::config::ID,
            system_program: system_program::ID,
            stake_program: stake::program::ID,
        }
    }

    pub fn resolve_for_prog_with_seeds(
        &self,
        program_id: Pubkey,
        RedelegateVoteAccounts { src, dst }: RedelegateVoteAccounts,
        RedelegateValidatorStakeSeeds {
            src_validator,
            dst_validator,
            src_transient,
            dst_transient,
            ephemeral,
        }: RedelegateValidatorStakeSeeds,
    ) -> RedelegateKeys {
        let [src_validator_stake_account, dst_validator_stake_account] =
            [(src, src_validator), (dst, dst_validator)].map(|(vote, seed)| {
                FindValidatorStakeAccount::new(FindValidatorStakeAccountArgs {
                    pool: self.pool.pubkey,
                    vote,
                    seed,
                })
                .run_for_prog(&program_id)
                .0
            });
        let [src_transient_stake_account, dst_transient_stake_account] =
            [(src, src_transient), (dst, dst_transient)].map(|(vote, seed)| {
                FindTransientStakeAccount::new(FindTransientStakeAccountArgs {
                    pool: self.pool.pubkey,
                    vote,
                    seed,
                })
                .run_for_prog(&program_id)
                .0
            });
        let pdas = RedelegatePdas {
            withdraw_authority: FindWithdrawAuthority {
                pool: self.pool.pubkey,
            }
            .run_for_prog(&program_id)
            .0,
            ephemeral_stake_account: FindEphemeralStakeAccount::new(
                FindEphemeralStakeAccountArgs {
                    pool: self.pool.pubkey,
                    seed: ephemeral,
                },
            )
            .run_for_prog(&program_id)
            .0,
            src_validator_stake_account,
            src_transient_stake_account,
            dst_validator_stake_account,
            dst_transient_stake_account,
        };
        self.resolve_with_pdas(dst, pdas)
    }
}
