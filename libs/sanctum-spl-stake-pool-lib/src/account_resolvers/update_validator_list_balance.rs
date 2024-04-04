use std::{cmp::min, num::NonZeroU32};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    stake, sysvar,
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_stake_pool_interface::{
    SplStakePoolProgramIx, StakePool, UpdateValidatorListBalanceIxArgs,
    UpdateValidatorListBalanceKeys, ValidatorStakeInfo,
    UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN,
};

use crate::{
    deserialize_stake_pool_checked, FindTransientStakeAccount, FindTransientStakeAccountArgs,
    FindValidatorStakeAccount, FindValidatorStakeAccountArgs, FindWithdrawAuthority,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpdateValidatorListBalance<P> {
    pub stake_pool: P,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpdateValidatorListBalanceFromSubsliceArgs {
    pub len: usize,
    pub start_index: usize,
    pub no_merge: bool,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkey> UpdateValidatorListBalance<P> {
    pub fn resolve_with_withdraw_auth(
        &self,
        withdraw_authority: Pubkey,
    ) -> Result<UpdateValidatorListBalanceKeys, ProgramError> {
        let StakePool {
            validator_list,
            reserve_stake,
            ..
        } = deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(UpdateValidatorListBalanceKeys {
            stake_pool: *self.stake_pool.pubkey(),
            withdraw_authority,
            validator_list,
            reserve_stake,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
            stake_program: stake::program::ID,
        })
    }

    pub fn resolve_for_prog(
        &self,
        program_id: &Pubkey,
    ) -> Result<UpdateValidatorListBalanceKeys, ProgramError> {
        let (withdraw_authority, _bump) = FindWithdrawAuthority {
            pool: *self.stake_pool.pubkey(),
        }
        .run_for_prog(program_id);
        self.resolve_with_withdraw_auth(withdraw_authority)
    }

    /// validator_pair_iter must yield (validator_stake_account, transient_stake_account)
    pub fn full_ix_with_validator_pairs<I: Iterator<Item = (Pubkey, Pubkey)>>(
        &self,
        program_id: Pubkey,
        validator_pair_iter: I,
        ix_args: UpdateValidatorListBalanceIxArgs,
    ) -> Result<Instruction, ProgramError> {
        let accounts: [AccountMeta; UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN] =
            self.resolve_for_prog(&program_id)?.into();
        let mut accounts = Vec::from(accounts);
        accounts.extend(validator_pair_iter.flat_map(|(vsa, tsa)| {
            [vsa, tsa].map(|pubkey| AccountMeta {
                pubkey,
                is_signer: false,
                is_writable: true,
            })
        }));
        Ok(Instruction {
            program_id,
            accounts,
            data: SplStakePoolProgramIx::UpdateValidatorListBalance(ix_args).try_to_vec()?,
        })
    }

    /// Creates an ix that updates all validators on `validator_slice`
    pub fn full_ix_from_validator_slice(
        &self,
        program_id: Pubkey,
        validator_slice: &[ValidatorStakeInfo],
        ix_args: UpdateValidatorListBalanceIxArgs,
    ) -> Result<Instruction, ProgramError> {
        self.full_ix_with_validator_pairs(
            program_id,
            validator_slice.iter().map(
                |ValidatorStakeInfo {
                     transient_seed_suffix,
                     validator_seed_suffix,
                     vote_account_address,
                     ..
                 }| {
                    let (vsa, _bump) =
                        FindValidatorStakeAccount::new(FindValidatorStakeAccountArgs {
                            pool: *self.stake_pool.pubkey(),
                            vote: *vote_account_address,
                            seed: NonZeroU32::new(*validator_seed_suffix),
                        })
                        .run_for_prog(&program_id);
                    let (tsa, _bump) =
                        FindTransientStakeAccount::new(FindTransientStakeAccountArgs {
                            pool: *self.stake_pool.pubkey(),
                            vote: *vote_account_address,
                            seed: *transient_seed_suffix,
                        })
                        .run_for_prog(&program_id);
                    (vsa, tsa)
                },
            ),
            ix_args,
        )
    }

    /// creates ix for `full_validator_list[ix_args.start_index..ix_args.start_index + len]`
    pub fn full_ix_from_validator_list_subslice(
        &self,
        program_id: Pubkey,
        full_validator_list: &[ValidatorStakeInfo],
        UpdateValidatorListBalanceFromSubsliceArgs {
            len,
            start_index,
            no_merge,
        }: UpdateValidatorListBalanceFromSubsliceArgs,
    ) -> Result<Instruction, ProgramError> {
        let end_index = min(
            full_validator_list.len(),
            start_index
                .checked_add(len)
                .ok_or(ProgramError::InvalidArgument)?,
        );
        self.full_ix_from_validator_slice(
            program_id,
            &full_validator_list[start_index..end_index],
            UpdateValidatorListBalanceIxArgs {
                start_index: start_index
                    .try_into()
                    .map_err(|_e| ProgramError::InvalidArgument)?,
                no_merge,
            },
        )
    }
}
