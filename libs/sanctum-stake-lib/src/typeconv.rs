//! For types repeated in IDL

use solana_program::stake::state::{Authorized, Lockup, StakeAuthorize};

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

impl From<NativeConv<Authorized>> for stake_program_interface::Authorized {
    fn from(NativeConv(Authorized { staker, withdrawer }): NativeConv<Authorized>) -> Self {
        Self { staker, withdrawer }
    }
}

impl From<NativeConv<stake_program_interface::Authorized>> for Authorized {
    fn from(
        NativeConv(stake_program_interface::Authorized { staker, withdrawer }): NativeConv<
            stake_program_interface::Authorized,
        >,
    ) -> Self {
        Self { staker, withdrawer }
    }
}

impl From<NativeConv<Lockup>> for stake_program_interface::Lockup {
    fn from(
        NativeConv(Lockup {
            unix_timestamp,
            epoch,
            custodian,
        }): NativeConv<Lockup>,
    ) -> Self {
        Self {
            unix_timestamp,
            epoch,
            custodian,
        }
    }
}

impl From<NativeConv<stake_program_interface::Lockup>> for Lockup {
    fn from(
        NativeConv(stake_program_interface::Lockup {
            unix_timestamp,
            epoch,
            custodian,
        }): NativeConv<stake_program_interface::Lockup>,
    ) -> Self {
        Self {
            unix_timestamp,
            epoch,
            custodian,
        }
    }
}
