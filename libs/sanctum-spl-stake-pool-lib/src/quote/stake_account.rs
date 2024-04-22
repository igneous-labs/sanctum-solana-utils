use borsh::BorshDeserialize;
use solana_program::{program_error::ProgramError, stake::state::StakeStateV2};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountLamports};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StakeAccountDataForQuoting {
    pub staked_lamports: u64,
    pub unstaked_lamports: u64,
}

impl StakeAccountDataForQuoting {
    pub fn from_stake_account<A: ReadonlyAccountData + ReadonlyAccountLamports>(
        stake_account: A,
    ) -> Result<Self, ProgramError> {
        let stake_state = StakeStateV2::deserialize(&mut stake_account.data().as_ref())?;
        let staked_lamports = stake_state.stake().map(|s| s.delegation.stake).unwrap_or(0);
        let unstaked_lamports = stake_account
            .lamports()
            .checked_sub(staked_lamports)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        Ok(Self {
            staked_lamports,
            unstaked_lamports,
        })
    }
}
