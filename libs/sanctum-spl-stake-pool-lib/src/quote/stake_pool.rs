use sanctum_token_ratio::{CeilDiv, FloorDiv, MathError, U64BpsFee, U64Ratio};
use spl_stake_pool_interface::StakePool;

use crate::{FeeToRatio, PctFeeToBpsFee};

pub trait QuoteStakePool {
    fn mint_ratio(&self) -> FloorDiv<U64Ratio<u64, u64>>;

    fn stake_deposit_fee_ratio(&self) -> CeilDiv<U64Ratio<u64, u64>>;

    fn sol_deposit_fee_ratio(&self) -> CeilDiv<U64Ratio<u64, u64>>;

    fn stake_referral_bps_fee(&self) -> Result<FloorDiv<U64BpsFee>, MathError>;
}

impl<T: QuoteStakePool> QuoteStakePool for &T {
    fn mint_ratio(&self) -> FloorDiv<U64Ratio<u64, u64>> {
        (*self).mint_ratio()
    }

    fn stake_deposit_fee_ratio(&self) -> CeilDiv<U64Ratio<u64, u64>> {
        (*self).stake_deposit_fee_ratio()
    }

    fn sol_deposit_fee_ratio(&self) -> CeilDiv<U64Ratio<u64, u64>> {
        (*self).sol_deposit_fee_ratio()
    }

    fn stake_referral_bps_fee(&self) -> Result<FloorDiv<U64BpsFee>, MathError> {
        (*self).stake_referral_bps_fee()
    }
}

impl QuoteStakePool for StakePool {
    fn mint_ratio(&self) -> FloorDiv<U64Ratio<u64, u64>> {
        FloorDiv(U64Ratio {
            num: self.pool_token_supply,
            denom: self.total_lamports,
        })
    }

    fn stake_deposit_fee_ratio(&self) -> CeilDiv<U64Ratio<u64, u64>> {
        self.stake_deposit_fee.fee_to_ratio()
    }

    fn sol_deposit_fee_ratio(&self) -> CeilDiv<U64Ratio<u64, u64>> {
        self.sol_deposit_fee.fee_to_ratio()
    }

    fn stake_referral_bps_fee(&self) -> Result<FloorDiv<U64BpsFee>, MathError> {
        self.stake_referral_fee.pct_fee_to_bps_fee()
    }
}
