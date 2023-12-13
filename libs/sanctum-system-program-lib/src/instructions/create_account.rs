use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub const CREATE_ACCOUNT_ACCOUNTS_LEN: usize = 2;

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountAccounts<'me, 'info> {
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountKeys {
    pub from: Pubkey,
    pub to: Pubkey,
}

impl CreateAccountKeys {
    pub fn to_ix(
        &self,
        CreateAccountArgs {
            space,
            owner,
            lamports,
        }: CreateAccountArgs,
    ) -> Instruction {
        system_instruction::create_account(&self.from, &self.to, lamports, space, &owner)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountArgs {
    pub space: u64,
    pub owner: Pubkey,
    pub lamports: u64,
}

impl From<CreateAccountAccounts<'_, '_>> for CreateAccountKeys {
    fn from(CreateAccountAccounts { from, to }: CreateAccountAccounts<'_, '_>) -> Self {
        Self {
            from: *from.key,
            to: *to.key,
        }
    }
}

impl<'info> From<CreateAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_ACCOUNT_ACCOUNTS_LEN]
{
    fn from(CreateAccountAccounts { from, to }: CreateAccountAccounts<'_, 'info>) -> Self {
        [from.clone(), to.clone()]
    }
}

pub fn create_account_invoke(
    accounts: CreateAccountAccounts,
    args: CreateAccountArgs,
) -> ProgramResult {
    let ix = CreateAccountKeys::from(accounts).to_ix(args);
    let account_infos: [AccountInfo; CREATE_ACCOUNT_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn create_account_invoke_signed(
    accounts: CreateAccountAccounts,
    args: CreateAccountArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = CreateAccountKeys::from(accounts).to_ix(args);
    let account_infos: [AccountInfo; CREATE_ACCOUNT_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}

#[derive(Clone, Copy, Debug)]
pub struct CreateRentExemptAccountArgs {
    pub space: usize,
    pub owner: Pubkey,
}

impl CreateRentExemptAccountArgs {
    pub fn try_calc_lamports(&self) -> Result<CreateAccountArgs, ProgramError> {
        let lamports = Rent::get()?.minimum_balance(self.space);
        let space_u64: u64 = self
            .space
            .try_into()
            .map_err(|_e| ProgramError::InvalidArgument)?;
        Ok(CreateAccountArgs {
            space: space_u64,
            owner: self.owner,
            lamports,
        })
    }
}

pub fn create_rent_exempt_account_invoke(
    accounts: CreateAccountAccounts,
    args: CreateRentExemptAccountArgs,
) -> ProgramResult {
    let args = args.try_calc_lamports()?;
    create_account_invoke(accounts, args)
}

pub fn create_rent_exempt_account_invoke_signed(
    accounts: CreateAccountAccounts,
    args: CreateRentExemptAccountArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let args = args.try_calc_lamports()?;
    create_account_invoke_signed(accounts, args, signer_seeds)
}
