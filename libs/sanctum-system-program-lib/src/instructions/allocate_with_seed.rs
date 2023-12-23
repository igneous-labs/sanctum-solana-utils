use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
};

pub const ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN: usize = 2;

#[derive(Clone, Copy, Debug)]
pub struct AllocateWithSeedAccounts<'me, 'info> {
    /// The base signer to allocate `to_allocate` from
    pub base: &'me AccountInfo<'info>,

    /// The account to allocate.
    /// `Pubkey::create_with_seed(base.key, <seed>, owner)`
    pub to_allocate: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AllocateWithSeedKeys {
    pub base: Pubkey,
    pub to_allocate: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AllocateWithSeedArgs<'a> {
    pub space: u64,
    pub owner: Pubkey,
    pub seed: &'a str,
}

impl From<AllocateWithSeedAccounts<'_, '_>> for AllocateWithSeedKeys {
    fn from(
        AllocateWithSeedAccounts { base, to_allocate }: AllocateWithSeedAccounts<'_, '_>,
    ) -> Self {
        Self {
            base: *base.key,
            to_allocate: *to_allocate.key,
        }
    }
}

impl<'info> From<AllocateWithSeedAccounts<'_, 'info>>
    for [AccountInfo<'info>; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN]
{
    fn from(
        AllocateWithSeedAccounts { base, to_allocate }: AllocateWithSeedAccounts<'_, 'info>,
    ) -> Self {
        [to_allocate.clone(), base.clone()]
    }
}

pub fn allocate_with_seed_ix(
    AllocateWithSeedKeys { base, to_allocate }: AllocateWithSeedKeys,
    AllocateWithSeedArgs { space, owner, seed }: AllocateWithSeedArgs,
) -> Instruction {
    system_instruction::allocate_with_seed(&to_allocate, &base, seed, space, &owner)
}

pub fn allocate_with_seed_invoke(
    accounts: AllocateWithSeedAccounts,
    args: AllocateWithSeedArgs,
) -> ProgramResult {
    let ix = allocate_with_seed_ix(AllocateWithSeedKeys::from(accounts), args);
    let account_infos: [AccountInfo; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn allocate_with_seed_invoke_signed(
    accounts: AllocateWithSeedAccounts,
    args: AllocateWithSeedArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = allocate_with_seed_ix(AllocateWithSeedKeys::from(accounts), args);
    let account_infos: [AccountInfo; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
