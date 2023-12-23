use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    system_instruction,
};

pub const ALLOCATE_IX_ACCOUNTS_LEN: usize = 1;

pub fn allocate_invoke(account: &AccountInfo, space: u64) -> ProgramResult {
    let ix = system_instruction::allocate(account.key, space);
    invoke(&ix, &[account.clone()])
}

pub fn allocate_invoke_signed(
    account: &AccountInfo,
    space: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = system_instruction::allocate(account.key, space);
    invoke_signed(&ix, &[account.clone()], signer_seeds)
}
