use std::num::NonZeroU32;

use solana_program::{pubkey::Pubkey, stake, sysvar};
use solana_readonly_account::keyed::Keyed;
use spl_stake_pool_interface::{StakePool, WithdrawStakeWithSlippageKeys};

use crate::{
    FindTransientStakeAccount, FindTransientStakeAccountArgs, FindValidatorStakeAccount,
    FindValidatorStakeAccountArgs, FindWithdrawAuthority,
};

#[derive(Clone, Copy, Debug)]
pub struct WithdrawStakeWithSlippage<'a> {
    pub pool: Keyed<&'a StakePool>,
    pub burn_from: Pubkey,
    /// Token account authority of `burn_from`
    pub transfer_authority: Pubkey,
    pub beneficiary: Pubkey,
    /// Needs to be initialized with 200 bytes of space, owner = stake program, and rent-exempt
    pub split_to: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WithdrawStakeComputedKeys {
    pub withdraw_authority_pda: Pubkey,
    pub split_from: Pubkey,
}

impl<'a> WithdrawStakeWithSlippage<'a> {
    pub fn compute_withdraw_auth(&self, program_id: &Pubkey) -> Pubkey {
        let (withdraw_authority_pda, _bump) = FindWithdrawAuthority {
            pool: self.pool.pubkey,
        }
        .run_for_prog(program_id);
        withdraw_authority_pda
    }

    pub fn compute_vsa(
        &self,
        program_id: &Pubkey,
        vote: Pubkey,
        validator_seed_suffix: u32,
    ) -> Pubkey {
        let (validator_stake_account, _bump) =
            FindValidatorStakeAccount::new(FindValidatorStakeAccountArgs {
                pool: self.pool.pubkey,
                vote,
                seed: NonZeroU32::new(validator_seed_suffix),
            })
            .run_for_prog(program_id);
        validator_stake_account
    }

    pub fn compute_tsa(
        &self,
        program_id: &Pubkey,
        vote: Pubkey,
        transient_seed_suffix: u64,
    ) -> Pubkey {
        let (transient_stake_account, _bump) =
            FindTransientStakeAccount::new(FindTransientStakeAccountArgs {
                pool: self.pool.pubkey,
                vote,
                seed: transient_seed_suffix,
            })
            .run_for_prog(program_id);
        transient_stake_account
    }

    pub fn compute_keys_for_reserve(&self, program_id: &Pubkey) -> WithdrawStakeComputedKeys {
        WithdrawStakeComputedKeys {
            withdraw_authority_pda: self.compute_withdraw_auth(program_id),
            split_from: self.pool.account.reserve_stake,
        }
    }

    pub fn compute_keys_for_vsa(
        &self,
        program_id: &Pubkey,
        vote: Pubkey,
        validator_seed_suffix: u32,
    ) -> WithdrawStakeComputedKeys {
        WithdrawStakeComputedKeys {
            withdraw_authority_pda: self.compute_withdraw_auth(program_id),
            split_from: self.compute_vsa(program_id, vote, validator_seed_suffix),
        }
    }

    pub fn compute_keys_for_tsa(
        &self,
        program_id: &Pubkey,
        vote: Pubkey,
        transient_seed_suffix: u64,
    ) -> WithdrawStakeComputedKeys {
        WithdrawStakeComputedKeys {
            withdraw_authority_pda: self.compute_withdraw_auth(program_id),
            split_from: self.compute_tsa(program_id, vote, transient_seed_suffix),
        }
    }

    pub fn resolve_with_computed_keys(
        &self,
        WithdrawStakeComputedKeys {
            withdraw_authority_pda,
            split_from,
        }: WithdrawStakeComputedKeys,
    ) -> WithdrawStakeWithSlippageKeys {
        let Self {
            pool:
                Keyed {
                    pubkey: stake_pool,
                    account:
                        StakePool {
                            validator_list,
                            pool_mint,
                            manager_fee_account,
                            token_program,
                            ..
                        },
                },
            burn_from,
            transfer_authority,
            beneficiary,
            split_to,
        } = self;
        WithdrawStakeWithSlippageKeys {
            stake_pool: *stake_pool,
            validator_list: *validator_list,
            withdraw_authority: withdraw_authority_pda,
            split_from,
            split_to: *split_to,
            beneficiary: *beneficiary,
            transfer_authority: *transfer_authority,
            burn_from: *burn_from,
            manager_fee_account: *manager_fee_account,
            pool_mint: *pool_mint,
            clock: sysvar::clock::ID,
            token_program: *token_program,
            stake_program: stake::program::ID,
        }
    }
}
