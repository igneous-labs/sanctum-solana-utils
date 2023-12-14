use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub const MINT_TO_ACCOUNTS_LEN: usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct MintToAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_to: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct MintToKeys {
    pub token_program: Pubkey,
    pub mint: Pubkey,
    pub mint_to: Pubkey,
    pub mint_authority: Pubkey,
}

impl From<MintToAccounts<'_, '_>> for MintToKeys {
    fn from(
        MintToAccounts {
            token_program,
            mint,
            mint_to,
            mint_authority,
        }: MintToAccounts<'_, '_>,
    ) -> Self {
        Self {
            token_program: *token_program.key,
            mint: *mint.key,
            mint_to: *mint_to.key,
            mint_authority: *mint_authority.key,
        }
    }
}

impl<'info> From<MintToAccounts<'_, 'info>> for [AccountInfo<'info>; MINT_TO_ACCOUNTS_LEN] {
    fn from(
        MintToAccounts {
            token_program: _,
            mint,
            mint_to,
            mint_authority,
        }: MintToAccounts<'_, 'info>,
    ) -> Self {
        [mint.clone(), mint_to.clone(), mint_authority.clone()]
    }
}

pub fn mint_to_ix(
    MintToKeys {
        token_program,
        mint,
        mint_to,
        mint_authority,
    }: MintToKeys,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    spl_token_2022::instruction::mint_to(
        &token_program,
        &mint,
        &mint_to,
        &mint_authority,
        &[],
        amount,
    )
}

pub fn mint_to_invoke(accounts: MintToAccounts, amount: u64) -> ProgramResult {
    let ix = mint_to_ix(MintToKeys::from(accounts), amount)?;
    let account_infos: [AccountInfo; MINT_TO_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn mint_to_invoke_signed(
    accounts: MintToAccounts,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = mint_to_ix(MintToKeys::from(accounts), amount)?;
    let account_infos: [AccountInfo; MINT_TO_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
