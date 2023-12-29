//! For types repeated in IDL

use solana_program::stake::state::StakeAuthorize;

pub struct NativeConv<T>(pub T);

impl From<NativeConv<StakeAuthorize>> for stake_program_interface::StakeAuthorize {
    fn from(NativeConv(a): NativeConv<StakeAuthorize>) -> Self {
        match a {
            StakeAuthorize::Staker => Self::Staker,
            StakeAuthorize::Withdrawer => Self::Withdrawer,
        }
    }
}

impl From<NativeConv<stake_program_interface::StakeAuthorize>> for StakeAuthorize {
    fn from(NativeConv(a): NativeConv<stake_program_interface::StakeAuthorize>) -> Self {
        match a {
            stake_program_interface::StakeAuthorize::Staker => Self::Staker,
            stake_program_interface::StakeAuthorize::Withdrawer => Self::Withdrawer,
        }
    }
}
