use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use spl_associated_token_account::instruction::create_associated_token_account;

pub const CREATE_ATA_ACCOUNTS_LEN: usize = 6;

#[derive(Clone, Copy, Debug)]
pub struct CreateAtaAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub ata_to_create: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct CreateAtaKeys {
    pub payer: Pubkey,
    pub ata_to_create: Pubkey,
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}

impl From<CreateAtaAccounts<'_, '_>> for CreateAtaKeys {
    fn from(
        CreateAtaAccounts {
            payer,
            ata_to_create,
            wallet,
            mint,
            system_program,
            token_program,
        }: CreateAtaAccounts<'_, '_>,
    ) -> Self {
        Self {
            payer: *payer.key,
            ata_to_create: *ata_to_create.key,
            wallet: *wallet.key,
            mint: *mint.key,
            system_program: *system_program.key,
            token_program: *token_program.key,
        }
    }
}

impl<'info> From<CreateAtaAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_ATA_ACCOUNTS_LEN] {
    fn from(
        CreateAtaAccounts {
            payer,
            ata_to_create,
            wallet,
            mint,
            system_program,
            token_program,
        }: CreateAtaAccounts<'_, 'info>,
    ) -> Self {
        [
            payer.clone(),
            ata_to_create.clone(),
            wallet.clone(),
            mint.clone(),
            system_program.clone(),
            token_program.clone(),
        ]
    }
}

pub fn create_ata_ix(
    CreateAtaKeys {
        payer,
        wallet,
        mint,
        system_program: _,
        ata_to_create: _,
        token_program,
    }: CreateAtaKeys,
) -> Instruction {
    create_associated_token_account(&payer, &wallet, &mint, &token_program)
}

pub fn create_ata_invoke(accounts: CreateAtaAccounts) -> ProgramResult {
    let ix = create_ata_ix(CreateAtaKeys::from(accounts));
    let account_infos: [AccountInfo; CREATE_ATA_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn create_ata_invoke_signed(
    accounts: CreateAtaAccounts,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_ata_ix(CreateAtaKeys::from(accounts));
    let account_infos: [AccountInfo; CREATE_ATA_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
