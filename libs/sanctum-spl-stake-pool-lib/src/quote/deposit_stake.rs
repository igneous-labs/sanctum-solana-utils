use sanctum_token_ratio::{ReversibleFee, ReversibleRatio};
use solana_program::program_error::ProgramError;
use spl_stake_pool_interface::StakePool;

use crate::{QuoteStakePool, StakeAccountDataForQuoting};

use super::DepositQuote;

pub trait QuoteDepositStake {
    fn quote_deposit_stake(
        &self,
        stake_account: &StakeAccountDataForQuoting,
    ) -> Result<DepositQuote, ProgramError>;
}

impl<T: QuoteDepositStake> QuoteDepositStake for &T {
    fn quote_deposit_stake(
        &self,
        stake_account: &StakeAccountDataForQuoting,
    ) -> Result<DepositQuote, ProgramError> {
        (*self).quote_deposit_stake(stake_account)
    }
}

impl QuoteDepositStake for StakePool {
    fn quote_deposit_stake(
        &self,
        StakeAccountDataForQuoting {
            staked_lamports,
            unstaked_lamports,
        }: &StakeAccountDataForQuoting,
    ) -> Result<DepositQuote, ProgramError> {
        // copied from
        // https://github.com/solana-labs/solana-program-library/blob/3e35101763097b5b3d21686191132e5d930f5b23/stake-pool/program/src/processor.rs#L2831-L2874

        let total_deposit_lamports = staked_lamports
            .checked_add(*unstaked_lamports)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let mint_ratio = self.mint_ratio();
        let new_pool_tokens = mint_ratio.apply(total_deposit_lamports)?;
        let new_pool_tokens_from_stake = mint_ratio.apply(*staked_lamports)?;
        let new_pool_tokens_from_sol = new_pool_tokens
            .checked_sub(new_pool_tokens_from_stake)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let stake_deposit_fee = self
            .stake_deposit_fee_ratio()?
            .apply(new_pool_tokens_from_stake)?
            .fee_charged();
        let sol_deposit_fee = self
            .sol_deposit_fee_ratio()?
            .apply(new_pool_tokens_from_sol)?
            .fee_charged();

        let total_fee = stake_deposit_fee
            .checked_add(sol_deposit_fee)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let user = new_pool_tokens
            .checked_sub(total_fee)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        if user == 0 {
            // DepositTooSmall
            return Err(ProgramError::InsufficientFunds);
        }

        let referrer = self
            .stake_referral_bps_fee()?
            .apply(total_fee)?
            .fee_charged();

        let manager = total_fee
            .checked_sub(referrer)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(DepositQuote {
            manager,
            referrer,
            user,
        })
    }
}
