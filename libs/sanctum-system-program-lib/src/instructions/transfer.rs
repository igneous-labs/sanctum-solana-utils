use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
};

pub const TRANSFER_IX_ACCOUNTS_LEN: usize = 2;

#[derive(Clone, Copy, Debug)]
pub struct TransferAccounts<'me, 'info> {
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransferKeys {
    pub from: Pubkey,
    pub to: Pubkey,
}

impl From<TransferAccounts<'_, '_>> for TransferKeys {
    fn from(TransferAccounts { from, to }: TransferAccounts<'_, '_>) -> Self {
        Self {
            from: *from.key,
            to: *to.key,
        }
    }
}

impl<'info> From<TransferAccounts<'_, 'info>> for [AccountInfo<'info>; TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(TransferAccounts { from, to }: TransferAccounts<'_, 'info>) -> Self {
        [from.clone(), to.clone()]
    }
}

pub fn transfer_ix(TransferKeys { from, to }: TransferKeys, lamports: u64) -> Instruction {
    system_instruction::transfer(&from, &to, lamports)
}

pub fn transfer_invoke(accounts: TransferAccounts, lamports: u64) -> ProgramResult {
    let ix = transfer_ix(TransferKeys::from(accounts), lamports);
    let account_infos: [AccountInfo; TRANSFER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn transfer_invoke_signed(
    accounts: TransferAccounts,
    lamports: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = transfer_ix(TransferKeys::from(accounts), lamports);
    let account_infos: [AccountInfo; TRANSFER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
