use std::num::NonZeroU32;

use solana_program::pubkey::{Pubkey, PubkeyError};

/// Seed for deposit authority seed
pub const AUTHORITY_DEPOSIT: &[u8] = b"deposit";

/// Seed for withdraw authority seed
pub const AUTHORITY_WITHDRAW: &[u8] = b"withdraw";

/// Seed for transient stake account
pub const TRANSIENT_STAKE_SEED_PREFIX: &[u8] = b"transient";

/// Seed for ephemeral stake account
pub const EPHEMERAL_STAKE_SEED_PREFIX: &[u8] = b"ephemeral";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindDepositAuthority {
    pub pool: Pubkey,
}

impl FindDepositAuthority {
    pub fn seeds(&self) -> [&[u8]; 2] {
        [self.pool.as_ref(), AUTHORITY_DEPOSIT]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateDepositAuthority {
    pub find: FindDepositAuthority,
    pub bump: u8,
}

impl CreateDepositAuthority {
    pub fn signer_seeds(&self) -> [&[u8]; 3] {
        let [s1, s2] = self.find.seeds();
        [s1, s2, std::slice::from_ref(&self.bump)]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(&self.signer_seeds(), program_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindWithdrawAuthority {
    pub pool: Pubkey,
}

impl FindWithdrawAuthority {
    pub fn seeds(&self) -> [&[u8]; 2] {
        [self.pool.as_ref(), AUTHORITY_WITHDRAW]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateWithdrawAuthority {
    pub find: FindWithdrawAuthority,
    pub bump: u8,
}

impl CreateWithdrawAuthority {
    pub fn signer_seeds(&self) -> [&[u8]; 3] {
        let [s1, s2] = self.find.seeds();
        [s1, s2, std::slice::from_ref(&self.bump)]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(&self.signer_seeds(), program_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindValidatorStakeAccountArgs {
    pub pool: Pubkey,
    pub vote: Pubkey,
    pub seed: Option<NonZeroU32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindValidatorStakeAccount {
    pub pool: Pubkey,
    pub vote: Pubkey,
    pub seed: Option<[u8; 4]>,
}

impl FindValidatorStakeAccount {
    pub fn new(
        FindValidatorStakeAccountArgs { pool, vote, seed }: FindValidatorStakeAccountArgs,
    ) -> Self {
        Self {
            pool,
            vote,
            seed: seed.map(|s| s.get().to_le_bytes()),
        }
    }

    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            self.vote.as_ref(),
            self.pool.as_ref(),
            self.seed.as_ref().map(|s| s.as_slice()).unwrap_or(&[]),
        ]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateValidatorStakeAccount {
    pub find: FindValidatorStakeAccount,
    pub bump: u8,
}

impl CreateValidatorStakeAccount {
    pub fn signer_seeds(&self) -> [&[u8]; 4] {
        let [s1, s2, s3] = self.find.seeds();
        [s1, s2, s3, std::slice::from_ref(&self.bump)]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(&self.signer_seeds(), program_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindTransientStakeAccountArgs {
    pub pool: Pubkey,
    pub vote: Pubkey,
    pub seed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindTransientStakeAccount {
    pub pool: Pubkey,
    pub vote: Pubkey,
    pub seed: [u8; 8],
}

impl FindTransientStakeAccount {
    pub fn new(
        FindTransientStakeAccountArgs { pool, vote, seed }: FindTransientStakeAccountArgs,
    ) -> Self {
        Self {
            pool,
            vote,
            seed: seed.to_le_bytes(),
        }
    }

    pub fn seeds(&self) -> [&[u8]; 4] {
        [
            TRANSIENT_STAKE_SEED_PREFIX,
            self.vote.as_ref(),
            self.pool.as_ref(),
            self.seed.as_ref(),
        ]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTransientStakeAccount {
    pub find: FindTransientStakeAccount,
    pub bump: u8,
}

impl CreateTransientStakeAccount {
    pub fn signer_seeds(&self) -> [&[u8]; 5] {
        let [s1, s2, s3, s4] = self.find.seeds();
        [s1, s2, s3, s4, std::slice::from_ref(&self.bump)]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(&self.signer_seeds(), program_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindEphemeralStakeAccountArgs {
    pub pool: Pubkey,
    pub seed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindEphemeralStakeAccount {
    pub pool: Pubkey,
    pub seed: [u8; 8],
}

impl FindEphemeralStakeAccount {
    pub fn new(
        FindEphemeralStakeAccountArgs { pool, seed }: FindEphemeralStakeAccountArgs,
    ) -> Self {
        Self {
            pool,
            seed: seed.to_le_bytes(),
        }
    }

    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            EPHEMERAL_STAKE_SEED_PREFIX,
            self.pool.as_ref(),
            self.seed.as_ref(),
        ]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatEphemeralStakeAccount {
    pub find: FindEphemeralStakeAccount,
    pub bump: u8,
}

impl CreatEphemeralStakeAccount {
    pub fn signer_seeds(&self) -> [&[u8]; 4] {
        let [s1, s2, s3] = self.find.seeds();
        [s1, s2, s3, std::slice::from_ref(&self.bump)]
    }

    pub fn run_for_prog(&self, program_id: &Pubkey) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(&self.signer_seeds(), program_id)
    }
}
