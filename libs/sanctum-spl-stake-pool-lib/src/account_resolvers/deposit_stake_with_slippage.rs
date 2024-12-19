use std::num::NonZeroU32;

use solana_program::{
    instruction::Instruction,
    program_error::ProgramError,
    pubkey::Pubkey,
    stake::{
        self,
        state::{Authorized, Lockup, StakeAuthorize, StakeStateV2},
    },
    sysvar,
};
use solana_readonly_account::keyed::Keyed;
use spl_stake_pool_interface::{
    deposit_stake_with_slippage_ix_with_program_id, DepositStakeWithSlippageIxArgs,
    DepositStakeWithSlippageKeys, StakePool,
};

use crate::{
    FindDepositAuthority, FindValidatorStakeAccount, FindValidatorStakeAccountArgs,
    FindWithdrawAuthority,
};

#[derive(Clone, Copy, Debug)]
pub struct DepositStakeWithSlippage<'a> {
    pub pool: Keyed<&'a StakePool>,
    /// The stake account to deposit. Must have authorities transferred to the pool's
    /// stake deposit authority before the actual DepositStake instruction is executed.
    /// This can be done using [`Self::stake_authorize_prefix_ixs`] or just use
    /// [`Self::full_ix_seq`] to get the fully formed instruction sequence
    pub stake_depositing: Keyed<&'a StakeStateV2>,
    pub mint_to: Pubkey,
    pub referral_fee_dest: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DepositStakeComputedKeys {
    pub deposit_authority_pda: Pubkey,
    pub withdraw_authority_pda: Pubkey,
    pub validator_stake_account: Pubkey,
}

impl<'a> DepositStakeWithSlippage<'a> {
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

    pub fn compute_deposit_authority_pda(&self, program_id: &Pubkey) -> Pubkey {
        let (deposit_authority_pda, _bump) = FindDepositAuthority {
            pool: self.pool.pubkey,
        }
        .run_for_prog(program_id);
        deposit_authority_pda
    }

    pub fn compute_keys(
        &self,
        program_id: &Pubkey,
        vote: Pubkey,
        validator_seed_suffix: u32, // obtained from ValidatorStakeInfo
    ) -> DepositStakeComputedKeys {
        DepositStakeComputedKeys {
            deposit_authority_pda: self.compute_deposit_authority_pda(program_id),
            withdraw_authority_pda: self.compute_withdraw_auth(program_id),
            validator_stake_account: self.compute_vsa(program_id, vote, validator_seed_suffix),
        }
    }

    pub fn full_ix_seq(
        &self,
        program_id: &Pubkey,
        vote: Pubkey,
        validator_seed_suffix: u32,
        min_tokens_out: u64,
    ) -> Result<[Instruction; 3], ProgramError> {
        let [stake_auth_staker, stake_auth_withdrawer] = self.stake_authorize_prefix_ixs()?;
        Ok([
            stake_auth_staker,
            stake_auth_withdrawer,
            self.full_ix(program_id, vote, validator_seed_suffix, min_tokens_out)?,
        ])
    }

    pub fn full_ix(
        &self,
        program_id: &Pubkey,
        vote: Pubkey,
        validator_seed_suffix: u32,
        min_tokens_out: u64,
    ) -> Result<Instruction, ProgramError> {
        const STAKE_DEPOSIT_AUTH_IDX: usize = 2;

        let computed_keys = self.compute_keys(program_id, vote, validator_seed_suffix);
        let mut ix = deposit_stake_with_slippage_ix_with_program_id(
            *program_id,
            self.resolve_with_computed_keys(computed_keys),
            DepositStakeWithSlippageIxArgs { min_tokens_out },
        )?;
        if ix.accounts[STAKE_DEPOSIT_AUTH_IDX].pubkey != computed_keys.deposit_authority_pda {
            ix.accounts[STAKE_DEPOSIT_AUTH_IDX].is_signer = true;
        }
        Ok(ix)
    }

    pub fn stake_authorize_prefix_ixs(&self) -> Result<[Instruction; 2], ProgramError> {
        let Authorized { staker, withdrawer } = self
            .stake_depositing
            .account
            .authorized()
            .ok_or(ProgramError::InvalidAccountData)?;
        let Lockup { custodian, .. } = self
            .stake_depositing
            .account
            .lockup()
            .ok_or(ProgramError::InvalidAccountData)?;
        let custodian = (custodian != Pubkey::default()).then_some(&custodian);
        Ok([
            stake::instruction::authorize(
                &self.stake_depositing.pubkey,
                &staker,
                &self.pool.account.stake_deposit_authority,
                StakeAuthorize::Staker,
                custodian,
            ),
            stake::instruction::authorize(
                &self.stake_depositing.pubkey,
                &withdrawer,
                &self.pool.account.stake_deposit_authority,
                StakeAuthorize::Withdrawer,
                custodian,
            ),
        ])
    }

    pub fn resolve_with_computed_keys(
        &self,
        DepositStakeComputedKeys {
            withdraw_authority_pda,
            validator_stake_account,
            deposit_authority_pda: _,
        }: DepositStakeComputedKeys,
    ) -> DepositStakeWithSlippageKeys {
        let Self {
            pool:
                Keyed {
                    pubkey: stake_pool,
                    account:
                        StakePool {
                            stake_deposit_authority,
                            validator_list,
                            reserve_stake,
                            pool_mint,
                            manager_fee_account,
                            token_program,
                            ..
                        },
                },
            stake_depositing,
            mint_to,
            referral_fee_dest,
        } = self;
        DepositStakeWithSlippageKeys {
            stake_pool: *stake_pool,
            validator_list: *validator_list,
            stake_deposit_authority: *stake_deposit_authority,
            withdraw_authority: withdraw_authority_pda,
            stake_depositing: stake_depositing.pubkey,
            validator_stake_account,
            reserve_stake: *reserve_stake,
            mint_to: *mint_to,
            manager_fee_account: *manager_fee_account,
            referral_fee_dest: *referral_fee_dest,
            pool_mint: *pool_mint,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
            token_program: *token_program,
            stake_program: stake::program::ID,
        }
    }
}
