use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum SplAssociatedTokenAccountProgramIx {
    Create,
    CreateIdempotent,
    RecoverNested,
}
impl SplAssociatedTokenAccountProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            CREATE_IX_DISCM => Ok(Self::Create),
            CREATE_IDEMPOTENT_IX_DISCM => Ok(Self::CreateIdempotent),
            RECOVER_NESTED_IX_DISCM => Ok(Self::RecoverNested),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Create => writer.write_all(&[CREATE_IX_DISCM]),
            Self::CreateIdempotent => writer.write_all(&[CREATE_IDEMPOTENT_IX_DISCM]),
            Self::RecoverNested => writer.write_all(&[RECOVER_NESTED_IX_DISCM]),
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccounts<'me, 'info> {
    ///Funding account (must be a system account)
    pub funding_account: &'me AccountInfo<'info>,
    ///Associated token account address to be created
    pub associated_token_account: &'me AccountInfo<'info>,
    ///Wallet address for the new associated token account
    pub wallet: &'me AccountInfo<'info>,
    ///The token mint for the new associated token account
    pub mint: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Wallet address for the new associated token account
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateKeys {
    ///Funding account (must be a system account)
    pub funding_account: Pubkey,
    ///Associated token account address to be created
    pub associated_token_account: Pubkey,
    ///Wallet address for the new associated token account
    pub wallet: Pubkey,
    ///The token mint for the new associated token account
    pub mint: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Wallet address for the new associated token account
    pub token_program: Pubkey,
}
impl From<CreateAccounts<'_, '_>> for CreateKeys {
    fn from(accounts: CreateAccounts) -> Self {
        Self {
            funding_account: *accounts.funding_account.key,
            associated_token_account: *accounts.associated_token_account.key,
            wallet: *accounts.wallet.key,
            mint: *accounts.mint.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CreateKeys> for [AccountMeta; CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_IX_ACCOUNTS_LEN]> for CreateKeys {
    fn from(pubkeys: [Pubkey; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funding_account: pubkeys[0],
            associated_token_account: pubkeys[1],
            wallet: pubkeys[2],
            mint: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAccounts<'_, 'info>) -> Self {
        [
            accounts.funding_account.clone(),
            accounts.associated_token_account.clone(),
            accounts.wallet.clone(),
            accounts.mint.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]>
    for CreateAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funding_account: &arr[0],
            associated_token_account: &arr[1],
            wallet: &arr[2],
            mint: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const CREATE_IX_DISCM: u8 = 0u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIxData;
impl CreateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CREATE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_ix(keys: CreateKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: CreateIxData.try_to_vec()?,
    })
}
pub fn create_invoke<'info>(accounts: CreateAccounts<'_, 'info>) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix(keys)?;
    let account_info: [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_invoke_signed<'info>(
    accounts: CreateAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix(keys)?;
    let account_info: [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn create_verify_account_keys(
    accounts: CreateAccounts<'_, '_>,
    keys: CreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.funding_account.key, &keys.funding_account),
        (
            accounts.associated_token_account.key,
            &keys.associated_token_account,
        ),
        (accounts.wallet.key, &keys.wallet),
        (accounts.mint.key, &keys.mint),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_verify_writable_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.funding_account, accounts.associated_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_verify_signer_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funding_account] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_verify_account_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_verify_writable_privileges(accounts)?;
    create_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateIdempotentAccounts<'me, 'info> {
    ///Funding account (must be a system account)
    pub funding_account: &'me AccountInfo<'info>,
    ///Associated token account address to be created
    pub associated_token_account: &'me AccountInfo<'info>,
    ///Wallet address for the new associated token account
    pub wallet: &'me AccountInfo<'info>,
    ///The token mint for the new associated token account
    pub mint: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Wallet address for the new associated token account
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateIdempotentKeys {
    ///Funding account (must be a system account)
    pub funding_account: Pubkey,
    ///Associated token account address to be created
    pub associated_token_account: Pubkey,
    ///Wallet address for the new associated token account
    pub wallet: Pubkey,
    ///The token mint for the new associated token account
    pub mint: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Wallet address for the new associated token account
    pub token_program: Pubkey,
}
impl From<CreateIdempotentAccounts<'_, '_>> for CreateIdempotentKeys {
    fn from(accounts: CreateIdempotentAccounts) -> Self {
        Self {
            funding_account: *accounts.funding_account.key,
            associated_token_account: *accounts.associated_token_account.key,
            wallet: *accounts.wallet.key,
            mint: *accounts.mint.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CreateIdempotentKeys> for [AccountMeta; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateIdempotentKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]> for CreateIdempotentKeys {
    fn from(pubkeys: [Pubkey; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funding_account: pubkeys[0],
            associated_token_account: pubkeys[1],
            wallet: pubkeys[2],
            mint: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateIdempotentAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateIdempotentAccounts<'_, 'info>) -> Self {
        [
            accounts.funding_account.clone(),
            accounts.associated_token_account.clone(),
            accounts.wallet.clone(),
            accounts.mint.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]>
    for CreateIdempotentAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            funding_account: &arr[0],
            associated_token_account: &arr[1],
            wallet: &arr[2],
            mint: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const CREATE_IDEMPOTENT_IX_DISCM: u8 = 1u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIdempotentIxData;
impl CreateIdempotentIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CREATE_IDEMPOTENT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_IDEMPOTENT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_IDEMPOTENT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_idempotent_ix(keys: CreateIdempotentKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: CreateIdempotentIxData.try_to_vec()?,
    })
}
pub fn create_idempotent_invoke<'info>(
    accounts: CreateIdempotentAccounts<'_, 'info>,
) -> ProgramResult {
    let keys: CreateIdempotentKeys = accounts.into();
    let ix = create_idempotent_ix(keys)?;
    let account_info: [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_idempotent_invoke_signed<'info>(
    accounts: CreateIdempotentAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateIdempotentKeys = accounts.into();
    let ix = create_idempotent_ix(keys)?;
    let account_info: [AccountInfo<'info>; CREATE_IDEMPOTENT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn create_idempotent_verify_account_keys(
    accounts: CreateIdempotentAccounts<'_, '_>,
    keys: CreateIdempotentKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.funding_account.key, &keys.funding_account),
        (
            accounts.associated_token_account.key,
            &keys.associated_token_account,
        ),
        (accounts.wallet.key, &keys.wallet),
        (accounts.mint.key, &keys.mint),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_idempotent_verify_writable_privileges<'me, 'info>(
    accounts: CreateIdempotentAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.funding_account, accounts.associated_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_idempotent_verify_signer_privileges<'me, 'info>(
    accounts: CreateIdempotentAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funding_account] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_idempotent_verify_account_privileges<'me, 'info>(
    accounts: CreateIdempotentAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_idempotent_verify_writable_privileges(accounts)?;
    create_idempotent_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const RECOVER_NESTED_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct RecoverNestedAccounts<'me, 'info> {
    ///Nested associated token account, must be owned by ownerAssociatedTokenAccount
    pub nested: &'me AccountInfo<'info>,
    ///Token mint for nested
    pub nested_mint: &'me AccountInfo<'info>,
    ///wallet's associated token account of nestedMint to recover the funds to, must be owned by wallet
    pub wallet_associated_token_account: &'me AccountInfo<'info>,
    ///wallet's associated token account of ownerAssociatedTokenAccountMint that owns nested
    pub owner_associated_token_account: &'me AccountInfo<'info>,
    ///Token mint for ownerAssociatedTokenAccount
    pub owner_token_account_mint: &'me AccountInfo<'info>,
    ///Wallet address for walletAssociatedTokenAccount
    pub wallet: &'me AccountInfo<'info>,
    ///SPL token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RecoverNestedKeys {
    ///Nested associated token account, must be owned by ownerAssociatedTokenAccount
    pub nested: Pubkey,
    ///Token mint for nested
    pub nested_mint: Pubkey,
    ///wallet's associated token account of nestedMint to recover the funds to, must be owned by wallet
    pub wallet_associated_token_account: Pubkey,
    ///wallet's associated token account of ownerAssociatedTokenAccountMint that owns nested
    pub owner_associated_token_account: Pubkey,
    ///Token mint for ownerAssociatedTokenAccount
    pub owner_token_account_mint: Pubkey,
    ///Wallet address for walletAssociatedTokenAccount
    pub wallet: Pubkey,
    ///SPL token program
    pub token_program: Pubkey,
}
impl From<RecoverNestedAccounts<'_, '_>> for RecoverNestedKeys {
    fn from(accounts: RecoverNestedAccounts) -> Self {
        Self {
            nested: *accounts.nested.key,
            nested_mint: *accounts.nested_mint.key,
            wallet_associated_token_account: *accounts.wallet_associated_token_account.key,
            owner_associated_token_account: *accounts.owner_associated_token_account.key,
            owner_token_account_mint: *accounts.owner_token_account_mint.key,
            wallet: *accounts.wallet.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RecoverNestedKeys> for [AccountMeta; RECOVER_NESTED_IX_ACCOUNTS_LEN] {
    fn from(keys: RecoverNestedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.nested,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.nested_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wallet_associated_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner_associated_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner_token_account_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; RECOVER_NESTED_IX_ACCOUNTS_LEN]> for RecoverNestedKeys {
    fn from(pubkeys: [Pubkey; RECOVER_NESTED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nested: pubkeys[0],
            nested_mint: pubkeys[1],
            wallet_associated_token_account: pubkeys[2],
            owner_associated_token_account: pubkeys[3],
            owner_token_account_mint: pubkeys[4],
            wallet: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<RecoverNestedAccounts<'_, 'info>>
    for [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RecoverNestedAccounts<'_, 'info>) -> Self {
        [
            accounts.nested.clone(),
            accounts.nested_mint.clone(),
            accounts.wallet_associated_token_account.clone(),
            accounts.owner_associated_token_account.clone(),
            accounts.owner_token_account_mint.clone(),
            accounts.wallet.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN]>
    for RecoverNestedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nested: &arr[0],
            nested_mint: &arr[1],
            wallet_associated_token_account: &arr[2],
            owner_associated_token_account: &arr[3],
            owner_token_account_mint: &arr[4],
            wallet: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const RECOVER_NESTED_IX_DISCM: u8 = 2u8;
#[derive(Clone, Debug, PartialEq)]
pub struct RecoverNestedIxData;
impl RecoverNestedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != RECOVER_NESTED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RECOVER_NESTED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[RECOVER_NESTED_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn recover_nested_ix(keys: RecoverNestedKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RECOVER_NESTED_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: RecoverNestedIxData.try_to_vec()?,
    })
}
pub fn recover_nested_invoke<'info>(accounts: RecoverNestedAccounts<'_, 'info>) -> ProgramResult {
    let keys: RecoverNestedKeys = accounts.into();
    let ix = recover_nested_ix(keys)?;
    let account_info: [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn recover_nested_invoke_signed<'info>(
    accounts: RecoverNestedAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RecoverNestedKeys = accounts.into();
    let ix = recover_nested_ix(keys)?;
    let account_info: [AccountInfo<'info>; RECOVER_NESTED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn recover_nested_verify_account_keys(
    accounts: RecoverNestedAccounts<'_, '_>,
    keys: RecoverNestedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.nested.key, &keys.nested),
        (accounts.nested_mint.key, &keys.nested_mint),
        (
            accounts.wallet_associated_token_account.key,
            &keys.wallet_associated_token_account,
        ),
        (
            accounts.owner_associated_token_account.key,
            &keys.owner_associated_token_account,
        ),
        (
            accounts.owner_token_account_mint.key,
            &keys.owner_token_account_mint,
        ),
        (accounts.wallet.key, &keys.wallet),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn recover_nested_verify_writable_privileges<'me, 'info>(
    accounts: RecoverNestedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.nested,
        accounts.wallet_associated_token_account,
        accounts.wallet,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn recover_nested_verify_signer_privileges<'me, 'info>(
    accounts: RecoverNestedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn recover_nested_verify_account_privileges<'me, 'info>(
    accounts: RecoverNestedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    recover_nested_verify_writable_privileges(accounts)?;
    recover_nested_verify_signer_privileges(accounts)?;
    Ok(())
}
