use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
};

pub const ASSIGN_IX_ACCOUNTS_LEN: usize = 1;

pub fn assign_invoke(account: &AccountInfo, owner: Pubkey) -> ProgramResult {
    let ix = system_instruction::assign(account.key, &owner);
    invoke(&ix, &[account.clone()])
}

pub fn assign_invoke_signed(
    account: &AccountInfo,
    owner: Pubkey,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = system_instruction::assign(account.key, &owner);
    invoke_signed(&ix, &[account.clone()], signer_seeds)
}
