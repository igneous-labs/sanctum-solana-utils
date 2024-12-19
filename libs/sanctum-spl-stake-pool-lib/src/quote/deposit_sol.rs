use sanctum_token_ratio::{ReversibleFee, ReversibleRatio};
use solana_program::program_error::ProgramError;
use spl_stake_pool_interface::StakePool;

use crate::QuoteStakePool;

use super::DepositQuote;

pub trait QuoteDepositSol {
    fn quote_deposit_sol(&self, lamports_in: u64) -> Result<DepositQuote, ProgramError>;
}

impl<T: QuoteDepositSol> QuoteDepositSol for &T {
    fn quote_deposit_sol(&self, lamports_in: u64) -> Result<DepositQuote, ProgramError> {
        (*self).quote_deposit_sol(lamports_in)
    }
}

impl QuoteDepositSol for StakePool {
    fn quote_deposit_sol(&self, lamports_in: u64) -> Result<DepositQuote, ProgramError> {
        // copied from
        // https://github.com/solana-labs/solana-program-library/blob/9165b84d544bc7e507822cb5ee5bcdefbb3e04a1/stake-pool/program/src/processor.rs#L2612-L2640

        let mint_ratio = self.mint_ratio();
        let new_pool_tokens = mint_ratio.apply(lamports_in)?;
        let after_sol_deposit_fee = self.sol_deposit_fee_ratio()?.apply(new_pool_tokens)?;
        let pool_tokens_sol_deposit_fee = after_sol_deposit_fee.fee_charged();
        let pool_tokens_user = after_sol_deposit_fee.amt_after_fee();
        let after_referral_fee = self
            .sol_referral_bps_fee()?
            .apply(pool_tokens_sol_deposit_fee)?;
        let pool_tokens_referral_fee = after_referral_fee.fee_charged();
        let pool_tokens_manager_deposit_fee = after_referral_fee.amt_after_fee();
        if pool_tokens_user == 0 {
            // DepositTooSmall
            return Err(ProgramError::InsufficientFunds);
        }
        Ok(DepositQuote {
            manager: pool_tokens_manager_deposit_fee,
            referrer: pool_tokens_referral_fee,
            user: pool_tokens_user,
        })
    }
}
