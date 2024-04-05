use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_stake_pool_interface::{StakePool, UpdateStakePoolBalanceKeys};

use crate::{deserialize_stake_pool_checked, FindWithdrawAuthority};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpdateStakePoolBalance<P> {
    pub stake_pool: P,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkey> UpdateStakePoolBalance<P> {
    pub fn resolve_with_withdraw_auth(
        &self,
        withdraw_authority: Pubkey,
    ) -> Result<UpdateStakePoolBalanceKeys, ProgramError> {
        let StakePool {
            validator_list,
            reserve_stake,
            pool_mint,
            manager_fee_account,
            token_program,
            ..
        } = deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(UpdateStakePoolBalanceKeys {
            stake_pool: *self.stake_pool.pubkey(),
            withdraw_authority,
            validator_list,
            reserve_stake,
            manager_fee_account,
            pool_mint,
            token_program,
        })
    }

    pub fn resolve_for_prog(
        &self,
        program_id: &Pubkey,
    ) -> Result<UpdateStakePoolBalanceKeys, ProgramError> {
        let (withdraw_authority, _bump) = FindWithdrawAuthority {
            pool: *self.stake_pool.pubkey(),
        }
        .run_for_prog(program_id);
        self.resolve_with_withdraw_auth(withdraw_authority)
    }
}
