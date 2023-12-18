use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_2022::instruction::burn;

pub const BURN_ACCOUNTS_LEN: usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct BurnAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub burn_from: &'me AccountInfo<'info>,
    pub burn_from_authority: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct BurnKeys {
    pub token_program: Pubkey,
    pub mint: Pubkey,
    pub burn_from: Pubkey,
    pub burn_from_authority: Pubkey,
}

impl From<BurnAccounts<'_, '_>> for BurnKeys {
    fn from(
        BurnAccounts {
            token_program,
            mint,
            burn_from,
            burn_from_authority,
        }: BurnAccounts<'_, '_>,
    ) -> Self {
        Self {
            token_program: *token_program.key,
            mint: *mint.key,
            burn_from: *burn_from.key,
            burn_from_authority: *burn_from_authority.key,
        }
    }
}

impl<'info> From<BurnAccounts<'_, 'info>> for [AccountInfo<'info>; BURN_ACCOUNTS_LEN] {
    fn from(
        BurnAccounts {
            token_program: _,
            mint,
            burn_from,
            burn_from_authority,
        }: BurnAccounts<'_, 'info>,
    ) -> Self {
        [burn_from.clone(), mint.clone(), burn_from_authority.clone()]
    }
}
impl BurnKeys {
    pub fn to_ix(&self, amount: u64) -> Result<Instruction, ProgramError> {
        burn(
            &self.token_program,
            &self.burn_from,
            &self.mint,
            &self.burn_from_authority,
            &[],
            amount,
        )
    }
}

pub fn burn_ix(
    BurnKeys {
        token_program,
        mint,
        burn_from,
        burn_from_authority,
    }: BurnKeys,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    burn(
        &token_program,
        &burn_from,
        &mint,
        &burn_from_authority,
        &[],
        amount,
    )
}

pub fn burn_invoke(accounts: BurnAccounts, amount: u64) -> ProgramResult {
    let ix = burn_ix(BurnKeys::from(accounts), amount)?;
    let account_infos: [AccountInfo; BURN_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn burn_invoke_signed(
    accounts: BurnAccounts,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = burn_ix(BurnKeys::from(accounts), amount)?;
    let account_infos: [AccountInfo; BURN_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
