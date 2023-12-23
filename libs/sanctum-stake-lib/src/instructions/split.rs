use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    stake::{self, instruction::StakeInstruction},
};

pub const SPLIT_IX_ACCOUNTS_LEN: usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct SplitAccounts<'me, 'info> {
    /// The stake account to split from
    pub from: &'me AccountInfo<'info>,

    /// The new stake account to split to.
    ///
    /// This must be an uninitialized stake account:
    /// - STAKE_ACCOUNT_LEN zero-allocated
    /// - owner set to stake program
    ///
    /// Starting from solana 1.17, this account must also be prefunded
    /// with at least rent-exempt minimum ([`crate::onchain_rent_exempt_lamports_for_stake_account()`])
    pub to: &'me AccountInfo<'info>,

    /// `from` stake account's stake authority.
    /// (`from.meta.authorized.staker`)
    pub staker: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct SplitKeys {
    pub from: Pubkey,
    pub to: Pubkey,
    pub staker: Pubkey,
}

impl From<SplitAccounts<'_, '_>> for SplitKeys {
    fn from(SplitAccounts { from, to, staker }: SplitAccounts<'_, '_>) -> Self {
        Self {
            from: *from.key,
            to: *to.key,
            staker: *staker.key,
        }
    }
}

impl<'info> From<SplitAccounts<'_, 'info>> for [AccountInfo<'info>; SPLIT_IX_ACCOUNTS_LEN] {
    fn from(SplitAccounts { from, to, staker }: SplitAccounts<'_, 'info>) -> Self {
        [from.clone(), to.clone(), staker.clone()]
    }
}

impl From<SplitKeys> for [AccountMeta; SPLIT_IX_ACCOUNTS_LEN] {
    fn from(SplitKeys { from, to, staker }: SplitKeys) -> Self {
        [
            AccountMeta {
                pubkey: from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: staker,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}

/// Args;
/// - `lamports`: number of staked lamports to split from `from` to `to`
pub fn split_ix(keys: SplitKeys, lamports: u64) -> Instruction {
    let metas: [AccountMeta; SPLIT_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        stake::program::ID,
        &StakeInstruction::Split(lamports),
        Vec::from(metas),
    )
}

pub fn split_invoke(accounts: SplitAccounts, lamports: u64) -> ProgramResult {
    let ix = split_ix(SplitKeys::from(accounts), lamports);
    let account_infos: [AccountInfo; SPLIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn split_invoke_signed(
    accounts: SplitAccounts,
    lamports: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = split_ix(SplitKeys::from(accounts), lamports);
    let account_infos: [AccountInfo; SPLIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
