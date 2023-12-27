use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
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
pub enum SplTokenProgramIx {
    InitializeMint(InitializeMintIxArgs),
    InitializeAccount,
    InitializeMultisig(InitializeMultisigIxArgs),
    Approve(ApproveIxArgs),
    Revoke,
    SetAuthority(SetAuthorityIxArgs),
    MintTo(MintToIxArgs),
    Burn(BurnIxArgs),
    CloseAccount,
    FreezeAccount,
    ThawAccount,
    TransferChecked(TransferCheckedIxArgs),
    ApproveChecked(ApproveCheckedIxArgs),
    MintToChecked(MintToCheckedIxArgs),
    BurnChecked(BurnCheckedIxArgs),
    InitializeAccount2(InitializeAccount2IxArgs),
    SyncNative,
    InitializeAccount3(InitializeAccount3IxArgs),
    InitializeMultisig2(InitializeMultisig2IxArgs),
    InitializeMint2(InitializeMint2IxArgs),
    GetTokenAccountDataSize,
    InitializeImmutableOwner,
    AmountToUiAmount(AmountToUiAmountIxArgs),
}
impl SplTokenProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            INITIALIZE_MINT_IX_DISCM => Ok(Self::InitializeMint(
                InitializeMintIxArgs::deserialize(&mut reader)?,
            )),
            INITIALIZE_ACCOUNT_IX_DISCM => Ok(Self::InitializeAccount),
            INITIALIZE_MULTISIG_IX_DISCM => Ok(Self::InitializeMultisig(
                InitializeMultisigIxArgs::deserialize(&mut reader)?,
            )),
            APPROVE_IX_DISCM => Ok(Self::Approve(ApproveIxArgs::deserialize(&mut reader)?)),
            REVOKE_IX_DISCM => Ok(Self::Revoke),
            SET_AUTHORITY_IX_DISCM => Ok(Self::SetAuthority(SetAuthorityIxArgs::deserialize(
                &mut reader,
            )?)),
            MINT_TO_IX_DISCM => Ok(Self::MintTo(MintToIxArgs::deserialize(&mut reader)?)),
            BURN_IX_DISCM => Ok(Self::Burn(BurnIxArgs::deserialize(&mut reader)?)),
            CLOSE_ACCOUNT_IX_DISCM => Ok(Self::CloseAccount),
            FREEZE_ACCOUNT_IX_DISCM => Ok(Self::FreezeAccount),
            THAW_ACCOUNT_IX_DISCM => Ok(Self::ThawAccount),
            TRANSFER_CHECKED_IX_DISCM => Ok(Self::TransferChecked(
                TransferCheckedIxArgs::deserialize(&mut reader)?,
            )),
            APPROVE_CHECKED_IX_DISCM => Ok(Self::ApproveChecked(
                ApproveCheckedIxArgs::deserialize(&mut reader)?,
            )),
            MINT_TO_CHECKED_IX_DISCM => Ok(Self::MintToChecked(MintToCheckedIxArgs::deserialize(
                &mut reader,
            )?)),
            BURN_CHECKED_IX_DISCM => Ok(Self::BurnChecked(BurnCheckedIxArgs::deserialize(
                &mut reader,
            )?)),
            INITIALIZE_ACCOUNT2_IX_DISCM => Ok(Self::InitializeAccount2(
                InitializeAccount2IxArgs::deserialize(&mut reader)?,
            )),
            SYNC_NATIVE_IX_DISCM => Ok(Self::SyncNative),
            INITIALIZE_ACCOUNT3_IX_DISCM => Ok(Self::InitializeAccount3(
                InitializeAccount3IxArgs::deserialize(&mut reader)?,
            )),
            INITIALIZE_MULTISIG2_IX_DISCM => Ok(Self::InitializeMultisig2(
                InitializeMultisig2IxArgs::deserialize(&mut reader)?,
            )),
            INITIALIZE_MINT2_IX_DISCM => Ok(Self::InitializeMint2(
                InitializeMint2IxArgs::deserialize(&mut reader)?,
            )),
            GET_TOKEN_ACCOUNT_DATA_SIZE_IX_DISCM => Ok(Self::GetTokenAccountDataSize),
            INITIALIZE_IMMUTABLE_OWNER_IX_DISCM => Ok(Self::InitializeImmutableOwner),
            AMOUNT_TO_UI_AMOUNT_IX_DISCM => Ok(Self::AmountToUiAmount(
                AmountToUiAmountIxArgs::deserialize(&mut reader)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeMint(args) => {
                writer.write_all(&[INITIALIZE_MINT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeAccount => writer.write_all(&[INITIALIZE_ACCOUNT_IX_DISCM]),
            Self::InitializeMultisig(args) => {
                writer.write_all(&[INITIALIZE_MULTISIG_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Approve(args) => {
                writer.write_all(&[APPROVE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Revoke => writer.write_all(&[REVOKE_IX_DISCM]),
            Self::SetAuthority(args) => {
                writer.write_all(&[SET_AUTHORITY_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::MintTo(args) => {
                writer.write_all(&[MINT_TO_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Burn(args) => {
                writer.write_all(&[BURN_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CloseAccount => writer.write_all(&[CLOSE_ACCOUNT_IX_DISCM]),
            Self::FreezeAccount => writer.write_all(&[FREEZE_ACCOUNT_IX_DISCM]),
            Self::ThawAccount => writer.write_all(&[THAW_ACCOUNT_IX_DISCM]),
            Self::TransferChecked(args) => {
                writer.write_all(&[TRANSFER_CHECKED_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::ApproveChecked(args) => {
                writer.write_all(&[APPROVE_CHECKED_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::MintToChecked(args) => {
                writer.write_all(&[MINT_TO_CHECKED_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::BurnChecked(args) => {
                writer.write_all(&[BURN_CHECKED_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeAccount2(args) => {
                writer.write_all(&[INITIALIZE_ACCOUNT2_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SyncNative => writer.write_all(&[SYNC_NATIVE_IX_DISCM]),
            Self::InitializeAccount3(args) => {
                writer.write_all(&[INITIALIZE_ACCOUNT3_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeMultisig2(args) => {
                writer.write_all(&[INITIALIZE_MULTISIG2_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeMint2(args) => {
                writer.write_all(&[INITIALIZE_MINT2_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::GetTokenAccountDataSize => {
                writer.write_all(&[GET_TOKEN_ACCOUNT_DATA_SIZE_IX_DISCM])
            }
            Self::InitializeImmutableOwner => {
                writer.write_all(&[INITIALIZE_IMMUTABLE_OWNER_IX_DISCM])
            }
            Self::AmountToUiAmount(args) => {
                writer.write_all(&[AMOUNT_TO_UI_AMOUNT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_MINT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct InitializeMintAccounts<'me, 'info> {
    ///The mint to initialize
    pub mint: &'me AccountInfo<'info>,
    ///Rent sysvar
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeMintKeys {
    ///The mint to initialize
    pub mint: Pubkey,
    ///Rent sysvar
    pub rent: Pubkey,
}
impl From<InitializeMintAccounts<'_, '_>> for InitializeMintKeys {
    fn from(accounts: InitializeMintAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeMintKeys> for [AccountMeta; INITIALIZE_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_MINT_IX_ACCOUNTS_LEN]> for InitializeMintKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            rent: pubkeys[1],
        }
    }
}
impl<'info> From<InitializeMintAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeMintAccounts<'_, 'info>) -> Self {
        [accounts.mint.clone(), accounts.rent.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_MINT_IX_ACCOUNTS_LEN]>
    for InitializeMintAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            rent: &arr[1],
        }
    }
}
pub const INITIALIZE_MINT_IX_DISCM: u8 = 0u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeMintIxArgs {
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMintIxData(pub InitializeMintIxArgs);
impl From<InitializeMintIxArgs> for InitializeMintIxData {
    fn from(args: InitializeMintIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_MINT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_MINT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeMintIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_MINT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_mint_ix(
    keys: InitializeMintKeys,
    args: InitializeMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeMintIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_mint_invoke<'info>(
    accounts: InitializeMintAccounts<'_, 'info>,
    args: InitializeMintIxArgs,
) -> ProgramResult {
    let keys: InitializeMintKeys = accounts.into();
    let ix = initialize_mint_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_mint_invoke_signed<'info>(
    accounts: InitializeMintAccounts<'_, 'info>,
    args: InitializeMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeMintKeys = accounts.into();
    let ix = initialize_mint_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_mint_verify_account_keys(
    accounts: InitializeMintAccounts<'_, '_>,
    keys: InitializeMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.mint.key, &keys.mint),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_mint_verify_writable_privileges<'me, 'info>(
    accounts: InitializeMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_mint_verify_account_privileges<'me, 'info>(
    accounts: InitializeMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_mint_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccountAccounts<'me, 'info> {
    ///The token account to initialize
    pub token_account: &'me AccountInfo<'info>,
    ///The mint this account will be associated with
    pub mint: &'me AccountInfo<'info>,
    ///The new account's owner/multisignaturer
    pub authority: &'me AccountInfo<'info>,
    ///Rent sysvar
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccountKeys {
    ///The token account to initialize
    pub token_account: Pubkey,
    ///The mint this account will be associated with
    pub mint: Pubkey,
    ///The new account's owner/multisignaturer
    pub authority: Pubkey,
    ///Rent sysvar
    pub rent: Pubkey,
}
impl From<InitializeAccountAccounts<'_, '_>> for InitializeAccountKeys {
    fn from(accounts: InitializeAccountAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            mint: *accounts.mint.key,
            authority: *accounts.authority.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeAccountKeys> for [AccountMeta; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN]> for InitializeAccountKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            mint: pubkeys[1],
            authority: pubkeys[2],
            rent: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.mint.clone(),
            accounts.authority.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for InitializeAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            mint: &arr[1],
            authority: &arr[2],
            rent: &arr[3],
        }
    }
}
pub const INITIALIZE_ACCOUNT_IX_DISCM: u8 = 1u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeAccountIxData;
impl InitializeAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_account_ix(keys: InitializeAccountKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: InitializeAccountIxData.try_to_vec()?,
    })
}
pub fn initialize_account_invoke<'info>(
    accounts: InitializeAccountAccounts<'_, 'info>,
) -> ProgramResult {
    let keys: InitializeAccountKeys = accounts.into();
    let ix = initialize_account_ix(keys)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_account_invoke_signed<'info>(
    accounts: InitializeAccountAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeAccountKeys = accounts.into();
    let ix = initialize_account_ix(keys)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_account_verify_account_keys(
    accounts: InitializeAccountAccounts<'_, '_>,
    keys: InitializeAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.authority.key, &keys.authority),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_account_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_account_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_account_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct InitializeMultisigAccounts<'me, 'info> {
    ///The multisignature account to initialize
    pub multisig: &'me AccountInfo<'info>,
    ///Rent sysvar. The signer accounts suffix slice follows. Length must equal to N where 1 <= N <= 11
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeMultisigKeys {
    ///The multisignature account to initialize
    pub multisig: Pubkey,
    ///Rent sysvar. The signer accounts suffix slice follows. Length must equal to N where 1 <= N <= 11
    pub rent: Pubkey,
}
impl From<InitializeMultisigAccounts<'_, '_>> for InitializeMultisigKeys {
    fn from(accounts: InitializeMultisigAccounts) -> Self {
        Self {
            multisig: *accounts.multisig.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeMultisigKeys> for [AccountMeta; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeMultisigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.multisig,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN]> for InitializeMultisigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            multisig: pubkeys[0],
            rent: pubkeys[1],
        }
    }
}
impl<'info> From<InitializeMultisigAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeMultisigAccounts<'_, 'info>) -> Self {
        [accounts.multisig.clone(), accounts.rent.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN]>
    for InitializeMultisigAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            multisig: &arr[0],
            rent: &arr[1],
        }
    }
}
pub const INITIALIZE_MULTISIG_IX_DISCM: u8 = 2u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeMultisigIxArgs {
    pub m: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMultisigIxData(pub InitializeMultisigIxArgs);
impl From<InitializeMultisigIxArgs> for InitializeMultisigIxData {
    fn from(args: InitializeMultisigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeMultisigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_MULTISIG_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_MULTISIG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeMultisigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_MULTISIG_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_multisig_ix(
    keys: InitializeMultisigKeys,
    args: InitializeMultisigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeMultisigIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_multisig_invoke<'info>(
    accounts: InitializeMultisigAccounts<'_, 'info>,
    args: InitializeMultisigIxArgs,
) -> ProgramResult {
    let keys: InitializeMultisigKeys = accounts.into();
    let ix = initialize_multisig_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_multisig_invoke_signed<'info>(
    accounts: InitializeMultisigAccounts<'_, 'info>,
    args: InitializeMultisigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeMultisigKeys = accounts.into();
    let ix = initialize_multisig_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_multisig_verify_account_keys(
    accounts: InitializeMultisigAccounts<'_, '_>,
    keys: InitializeMultisigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.multisig.key, &keys.multisig),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_multisig_verify_writable_privileges<'me, 'info>(
    accounts: InitializeMultisigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.multisig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_multisig_verify_account_privileges<'me, 'info>(
    accounts: InitializeMultisigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_multisig_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const APPROVE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ApproveAccounts<'me, 'info> {
    ///The token account to approve spending of
    pub token_account: &'me AccountInfo<'info>,
    ///The delegate to approve spending to
    pub delegate: &'me AccountInfo<'info>,
    ///The source token account's authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveKeys {
    ///The token account to approve spending of
    pub token_account: Pubkey,
    ///The delegate to approve spending to
    pub delegate: Pubkey,
    ///The source token account's authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<ApproveAccounts<'_, '_>> for ApproveKeys {
    fn from(accounts: ApproveAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            delegate: *accounts.delegate.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<ApproveKeys> for [AccountMeta; APPROVE_IX_ACCOUNTS_LEN] {
    fn from(keys: ApproveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.delegate,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; APPROVE_IX_ACCOUNTS_LEN]> for ApproveKeys {
    fn from(pubkeys: [Pubkey; APPROVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            delegate: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<ApproveAccounts<'_, 'info>> for [AccountInfo<'info>; APPROVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ApproveAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.delegate.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; APPROVE_IX_ACCOUNTS_LEN]>
    for ApproveAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; APPROVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            delegate: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const APPROVE_IX_DISCM: u8 = 4u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApproveIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveIxData(pub ApproveIxArgs);
impl From<ApproveIxArgs> for ApproveIxData {
    fn from(args: ApproveIxArgs) -> Self {
        Self(args)
    }
}
impl ApproveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != APPROVE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    APPROVE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ApproveIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[APPROVE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn approve_ix(keys: ApproveKeys, args: ApproveIxArgs) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; APPROVE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ApproveIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn approve_invoke<'info>(
    accounts: ApproveAccounts<'_, 'info>,
    args: ApproveIxArgs,
) -> ProgramResult {
    let keys: ApproveKeys = accounts.into();
    let ix = approve_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; APPROVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn approve_invoke_signed<'info>(
    accounts: ApproveAccounts<'_, 'info>,
    args: ApproveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ApproveKeys = accounts.into();
    let ix = approve_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; APPROVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn approve_verify_account_keys(
    accounts: ApproveAccounts<'_, '_>,
    keys: ApproveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.delegate.key, &keys.delegate),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn approve_verify_writable_privileges<'me, 'info>(
    accounts: ApproveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn approve_verify_signer_privileges<'me, 'info>(
    accounts: ApproveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn approve_verify_account_privileges<'me, 'info>(
    accounts: ApproveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    approve_verify_writable_privileges(accounts)?;
    approve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RevokeAccounts<'me, 'info> {
    ///The source token account
    pub token_account: &'me AccountInfo<'info>,
    ///The source token account's authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RevokeKeys {
    ///The source token account
    pub token_account: Pubkey,
    ///The source token account's authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<RevokeAccounts<'_, '_>> for RevokeKeys {
    fn from(accounts: RevokeAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<RevokeKeys> for [AccountMeta; REVOKE_IX_ACCOUNTS_LEN] {
    fn from(keys: RevokeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REVOKE_IX_ACCOUNTS_LEN]> for RevokeKeys {
    fn from(pubkeys: [Pubkey; REVOKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<RevokeAccounts<'_, 'info>> for [AccountInfo<'info>; REVOKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RevokeAccounts<'_, 'info>) -> Self {
        [accounts.token_account.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REVOKE_IX_ACCOUNTS_LEN]>
    for RevokeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REVOKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const REVOKE_IX_DISCM: u8 = 5u8;
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeIxData;
impl RevokeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != REVOKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REVOKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REVOKE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_ix(keys: RevokeKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: RevokeIxData.try_to_vec()?,
    })
}
pub fn revoke_invoke<'info>(accounts: RevokeAccounts<'_, 'info>) -> ProgramResult {
    let keys: RevokeKeys = accounts.into();
    let ix = revoke_ix(keys)?;
    let account_info: [AccountInfo<'info>; REVOKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn revoke_invoke_signed<'info>(
    accounts: RevokeAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeKeys = accounts.into();
    let ix = revoke_ix(keys)?;
    let account_info: [AccountInfo<'info>; REVOKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn revoke_verify_account_keys(
    accounts: RevokeAccounts<'_, '_>,
    keys: RevokeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn revoke_verify_writable_privileges<'me, 'info>(
    accounts: RevokeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_verify_signer_privileges<'me, 'info>(
    accounts: RevokeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_verify_account_privileges<'me, 'info>(
    accounts: RevokeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_verify_writable_privileges(accounts)?;
    revoke_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetAuthorityAccounts<'me, 'info> {
    ///The mint or account to change the authority of
    pub account: &'me AccountInfo<'info>,
    ///The current authority of the mint or account. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetAuthorityKeys {
    ///The mint or account to change the authority of
    pub account: Pubkey,
    ///The current authority of the mint or account. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<SetAuthorityAccounts<'_, '_>> for SetAuthorityKeys {
    fn from(accounts: SetAuthorityAccounts) -> Self {
        Self {
            account: *accounts.account.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetAuthorityKeys> for [AccountMeta; SET_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_AUTHORITY_IX_ACCOUNTS_LEN]> for SetAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            account: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.account.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_AUTHORITY_IX_ACCOUNTS_LEN]>
    for SetAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            account: &arr[0],
            authority: &arr[1],
        }
    }
}
pub const SET_AUTHORITY_IX_DISCM: u8 = 6u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetAuthorityIxArgs {
    pub authority_type: AuthorityType,
    pub new_authority: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetAuthorityIxData(pub SetAuthorityIxArgs);
impl From<SetAuthorityIxArgs> for SetAuthorityIxData {
    fn from(args: SetAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl SetAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetAuthorityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_AUTHORITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_authority_ix(
    keys: SetAuthorityKeys,
    args: SetAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetAuthorityIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_authority_invoke<'info>(
    accounts: SetAuthorityAccounts<'_, 'info>,
    args: SetAuthorityIxArgs,
) -> ProgramResult {
    let keys: SetAuthorityKeys = accounts.into();
    let ix = set_authority_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; SET_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_authority_invoke_signed<'info>(
    accounts: SetAuthorityAccounts<'_, 'info>,
    args: SetAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetAuthorityKeys = accounts.into();
    let ix = set_authority_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; SET_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_authority_verify_account_keys(
    accounts: SetAuthorityAccounts<'_, '_>,
    keys: SetAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.account.key, &keys.account),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_authority_verify_account_privileges<'me, 'info>(
    accounts: SetAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_authority_verify_writable_privileges(accounts)?;
    set_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_TO_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MintToAccounts<'me, 'info> {
    ///tokenAccount's mint
    pub mint: &'me AccountInfo<'info>,
    ///The token account to mint tokens to
    pub token_account: &'me AccountInfo<'info>,
    ///The mint authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct MintToKeys {
    ///tokenAccount's mint
    pub mint: Pubkey,
    ///The token account to mint tokens to
    pub token_account: Pubkey,
    ///The mint authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<MintToAccounts<'_, '_>> for MintToKeys {
    fn from(accounts: MintToAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            token_account: *accounts.token_account.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<MintToKeys> for [AccountMeta; MINT_TO_IX_ACCOUNTS_LEN] {
    fn from(keys: MintToKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_TO_IX_ACCOUNTS_LEN]> for MintToKeys {
    fn from(pubkeys: [Pubkey; MINT_TO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            token_account: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<MintToAccounts<'_, 'info>> for [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN] {
    fn from(accounts: MintToAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.token_account.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN]>
    for MintToAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            token_account: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const MINT_TO_IX_DISCM: u8 = 7u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MintToIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintToIxData(pub MintToIxArgs);
impl From<MintToIxArgs> for MintToIxData {
    fn from(args: MintToIxArgs) -> Self {
        Self(args)
    }
}
impl MintToIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != MINT_TO_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MINT_TO_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MintToIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MINT_TO_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_to_ix(keys: MintToKeys, args: MintToIxArgs) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_TO_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintToIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_to_invoke<'info>(
    accounts: MintToAccounts<'_, 'info>,
    args: MintToIxArgs,
) -> ProgramResult {
    let keys: MintToKeys = accounts.into();
    let ix = mint_to_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn mint_to_invoke_signed<'info>(
    accounts: MintToAccounts<'_, 'info>,
    args: MintToIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintToKeys = accounts.into();
    let ix = mint_to_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; MINT_TO_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn mint_to_verify_account_keys(
    accounts: MintToAccounts<'_, '_>,
    keys: MintToKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.mint.key, &keys.mint),
        (accounts.token_account.key, &keys.token_account),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn mint_to_verify_writable_privileges<'me, 'info>(
    accounts: MintToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.mint, accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_to_verify_signer_privileges<'me, 'info>(
    accounts: MintToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_to_verify_account_privileges<'me, 'info>(
    accounts: MintToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_to_verify_writable_privileges(accounts)?;
    mint_to_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BURN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct BurnAccounts<'me, 'info> {
    ///The token account to burn tokens from
    pub token_account: &'me AccountInfo<'info>,
    ///tokenAccount's mint
    pub mint: &'me AccountInfo<'info>,
    ///tokenAccount's authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct BurnKeys {
    ///The token account to burn tokens from
    pub token_account: Pubkey,
    ///tokenAccount's mint
    pub mint: Pubkey,
    ///tokenAccount's authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<BurnAccounts<'_, '_>> for BurnKeys {
    fn from(accounts: BurnAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            mint: *accounts.mint.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<BurnKeys> for [AccountMeta; BURN_IX_ACCOUNTS_LEN] {
    fn from(keys: BurnKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BURN_IX_ACCOUNTS_LEN]> for BurnKeys {
    fn from(pubkeys: [Pubkey; BURN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            mint: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<BurnAccounts<'_, 'info>> for [AccountInfo<'info>; BURN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BurnAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.mint.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BURN_IX_ACCOUNTS_LEN]>
    for BurnAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; BURN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            mint: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const BURN_IX_DISCM: u8 = 8u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BurnIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnIxData(pub BurnIxArgs);
impl From<BurnIxArgs> for BurnIxData {
    fn from(args: BurnIxArgs) -> Self {
        Self(args)
    }
}
impl BurnIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != BURN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BURN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(BurnIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[BURN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn burn_ix(keys: BurnKeys, args: BurnIxArgs) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BURN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BurnIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn burn_invoke<'info>(accounts: BurnAccounts<'_, 'info>, args: BurnIxArgs) -> ProgramResult {
    let keys: BurnKeys = accounts.into();
    let ix = burn_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; BURN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn burn_invoke_signed<'info>(
    accounts: BurnAccounts<'_, 'info>,
    args: BurnIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BurnKeys = accounts.into();
    let ix = burn_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; BURN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn burn_verify_account_keys(
    accounts: BurnAccounts<'_, '_>,
    keys: BurnKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn burn_verify_writable_privileges<'me, 'info>(
    accounts: BurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account, accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn burn_verify_signer_privileges<'me, 'info>(
    accounts: BurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn burn_verify_account_privileges<'me, 'info>(
    accounts: BurnAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    burn_verify_writable_privileges(accounts)?;
    burn_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseAccountAccounts<'me, 'info> {
    ///The token account to close
    pub token_account: &'me AccountInfo<'info>,
    ///The destination account to refund tokenAccount's SOL balance to
    pub dst: &'me AccountInfo<'info>,
    ///The token account's close authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CloseAccountKeys {
    ///The token account to close
    pub token_account: Pubkey,
    ///The destination account to refund tokenAccount's SOL balance to
    pub dst: Pubkey,
    ///The token account's close authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<CloseAccountAccounts<'_, '_>> for CloseAccountKeys {
    fn from(accounts: CloseAccountAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            dst: *accounts.dst.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<CloseAccountKeys> for [AccountMeta; CLOSE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dst,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_ACCOUNT_IX_ACCOUNTS_LEN]> for CloseAccountKeys {
    fn from(pubkeys: [Pubkey; CLOSE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            dst: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<CloseAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLOSE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CloseAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.dst.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for CloseAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            dst: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const CLOSE_ACCOUNT_IX_DISCM: u8 = 9u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CloseAccountIxData;
impl CloseAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CLOSE_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLOSE_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CLOSE_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_account_ix(keys: CloseAccountKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: CloseAccountIxData.try_to_vec()?,
    })
}
pub fn close_account_invoke<'info>(accounts: CloseAccountAccounts<'_, 'info>) -> ProgramResult {
    let keys: CloseAccountKeys = accounts.into();
    let ix = close_account_ix(keys)?;
    let account_info: [AccountInfo<'info>; CLOSE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn close_account_invoke_signed<'info>(
    accounts: CloseAccountAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseAccountKeys = accounts.into();
    let ix = close_account_ix(keys)?;
    let account_info: [AccountInfo<'info>; CLOSE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn close_account_verify_account_keys(
    accounts: CloseAccountAccounts<'_, '_>,
    keys: CloseAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.dst.key, &keys.dst),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn close_account_verify_writable_privileges<'me, 'info>(
    accounts: CloseAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account, accounts.dst] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_account_verify_signer_privileges<'me, 'info>(
    accounts: CloseAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_account_verify_account_privileges<'me, 'info>(
    accounts: CloseAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_account_verify_writable_privileges(accounts)?;
    close_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FREEZE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct FreezeAccountAccounts<'me, 'info> {
    ///The token account to freeze
    pub token_account: &'me AccountInfo<'info>,
    ///tokenAccount's mint
    pub mint: &'me AccountInfo<'info>,
    ///The mint's freeze authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct FreezeAccountKeys {
    ///The token account to freeze
    pub token_account: Pubkey,
    ///tokenAccount's mint
    pub mint: Pubkey,
    ///The mint's freeze authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<FreezeAccountAccounts<'_, '_>> for FreezeAccountKeys {
    fn from(accounts: FreezeAccountAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            mint: *accounts.mint.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<FreezeAccountKeys> for [AccountMeta; FREEZE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: FreezeAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FREEZE_ACCOUNT_IX_ACCOUNTS_LEN]> for FreezeAccountKeys {
    fn from(pubkeys: [Pubkey; FREEZE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            mint: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<FreezeAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; FREEZE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: FreezeAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.mint.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FREEZE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for FreezeAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; FREEZE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            mint: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const FREEZE_ACCOUNT_IX_DISCM: u8 = 10u8;
#[derive(Clone, Debug, PartialEq)]
pub struct FreezeAccountIxData;
impl FreezeAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != FREEZE_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FREEZE_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[FREEZE_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn freeze_account_ix(keys: FreezeAccountKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FREEZE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: FreezeAccountIxData.try_to_vec()?,
    })
}
pub fn freeze_account_invoke<'info>(accounts: FreezeAccountAccounts<'_, 'info>) -> ProgramResult {
    let keys: FreezeAccountKeys = accounts.into();
    let ix = freeze_account_ix(keys)?;
    let account_info: [AccountInfo<'info>; FREEZE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn freeze_account_invoke_signed<'info>(
    accounts: FreezeAccountAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FreezeAccountKeys = accounts.into();
    let ix = freeze_account_ix(keys)?;
    let account_info: [AccountInfo<'info>; FREEZE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn freeze_account_verify_account_keys(
    accounts: FreezeAccountAccounts<'_, '_>,
    keys: FreezeAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn freeze_account_verify_writable_privileges<'me, 'info>(
    accounts: FreezeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn freeze_account_verify_signer_privileges<'me, 'info>(
    accounts: FreezeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn freeze_account_verify_account_privileges<'me, 'info>(
    accounts: FreezeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    freeze_account_verify_writable_privileges(accounts)?;
    freeze_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const THAW_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ThawAccountAccounts<'me, 'info> {
    ///The frozen token account to thaw
    pub token_account: &'me AccountInfo<'info>,
    ///tokenAccount's mint
    pub mint: &'me AccountInfo<'info>,
    ///The mint's freeze authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ThawAccountKeys {
    ///The frozen token account to thaw
    pub token_account: Pubkey,
    ///tokenAccount's mint
    pub mint: Pubkey,
    ///The mint's freeze authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<ThawAccountAccounts<'_, '_>> for ThawAccountKeys {
    fn from(accounts: ThawAccountAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            mint: *accounts.mint.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<ThawAccountKeys> for [AccountMeta; THAW_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: ThawAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; THAW_ACCOUNT_IX_ACCOUNTS_LEN]> for ThawAccountKeys {
    fn from(pubkeys: [Pubkey; THAW_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            mint: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<ThawAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; THAW_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ThawAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.mint.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; THAW_ACCOUNT_IX_ACCOUNTS_LEN]>
    for ThawAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; THAW_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            mint: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const THAW_ACCOUNT_IX_DISCM: u8 = 11u8;
#[derive(Clone, Debug, PartialEq)]
pub struct ThawAccountIxData;
impl ThawAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != THAW_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    THAW_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[THAW_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn thaw_account_ix(keys: ThawAccountKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; THAW_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: ThawAccountIxData.try_to_vec()?,
    })
}
pub fn thaw_account_invoke<'info>(accounts: ThawAccountAccounts<'_, 'info>) -> ProgramResult {
    let keys: ThawAccountKeys = accounts.into();
    let ix = thaw_account_ix(keys)?;
    let account_info: [AccountInfo<'info>; THAW_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn thaw_account_invoke_signed<'info>(
    accounts: ThawAccountAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ThawAccountKeys = accounts.into();
    let ix = thaw_account_ix(keys)?;
    let account_info: [AccountInfo<'info>; THAW_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn thaw_account_verify_account_keys(
    accounts: ThawAccountAccounts<'_, '_>,
    keys: ThawAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn thaw_account_verify_writable_privileges<'me, 'info>(
    accounts: ThawAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn thaw_account_verify_signer_privileges<'me, 'info>(
    accounts: ThawAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn thaw_account_verify_account_privileges<'me, 'info>(
    accounts: ThawAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    thaw_account_verify_writable_privileges(accounts)?;
    thaw_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_CHECKED_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct TransferCheckedAccounts<'me, 'info> {
    ///The source token account to transfer from
    pub src: &'me AccountInfo<'info>,
    ///The token mint
    pub mint: &'me AccountInfo<'info>,
    ///The destination token account to transfer to
    pub dst: &'me AccountInfo<'info>,
    ///src's authority/delegate. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct TransferCheckedKeys {
    ///The source token account to transfer from
    pub src: Pubkey,
    ///The token mint
    pub mint: Pubkey,
    ///The destination token account to transfer to
    pub dst: Pubkey,
    ///src's authority/delegate. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<TransferCheckedAccounts<'_, '_>> for TransferCheckedKeys {
    fn from(accounts: TransferCheckedAccounts) -> Self {
        Self {
            src: *accounts.src.key,
            mint: *accounts.mint.key,
            dst: *accounts.dst.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<TransferCheckedKeys> for [AccountMeta; TRANSFER_CHECKED_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferCheckedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.src,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dst,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_CHECKED_IX_ACCOUNTS_LEN]> for TransferCheckedKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            src: pubkeys[0],
            mint: pubkeys[1],
            dst: pubkeys[2],
            authority: pubkeys[3],
        }
    }
}
impl<'info> From<TransferCheckedAccounts<'_, 'info>>
    for [AccountInfo<'info>; TRANSFER_CHECKED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: TransferCheckedAccounts<'_, 'info>) -> Self {
        [
            accounts.src.clone(),
            accounts.mint.clone(),
            accounts.dst.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_CHECKED_IX_ACCOUNTS_LEN]>
    for TransferCheckedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            src: &arr[0],
            mint: &arr[1],
            dst: &arr[2],
            authority: &arr[3],
        }
    }
}
pub const TRANSFER_CHECKED_IX_DISCM: u8 = 12u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransferCheckedIxArgs {
    pub args: CheckedOpArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferCheckedIxData(pub TransferCheckedIxArgs);
impl From<TransferCheckedIxArgs> for TransferCheckedIxData {
    fn from(args: TransferCheckedIxArgs) -> Self {
        Self(args)
    }
}
impl TransferCheckedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != TRANSFER_CHECKED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TRANSFER_CHECKED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(TransferCheckedIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[TRANSFER_CHECKED_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_checked_ix(
    keys: TransferCheckedKeys,
    args: TransferCheckedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_CHECKED_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferCheckedIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_checked_invoke<'info>(
    accounts: TransferCheckedAccounts<'_, 'info>,
    args: TransferCheckedIxArgs,
) -> ProgramResult {
    let keys: TransferCheckedKeys = accounts.into();
    let ix = transfer_checked_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; TRANSFER_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn transfer_checked_invoke_signed<'info>(
    accounts: TransferCheckedAccounts<'_, 'info>,
    args: TransferCheckedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferCheckedKeys = accounts.into();
    let ix = transfer_checked_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; TRANSFER_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn transfer_checked_verify_account_keys(
    accounts: TransferCheckedAccounts<'_, '_>,
    keys: TransferCheckedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.src.key, &keys.src),
        (accounts.mint.key, &keys.mint),
        (accounts.dst.key, &keys.dst),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn transfer_checked_verify_writable_privileges<'me, 'info>(
    accounts: TransferCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.src, accounts.dst] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_checked_verify_signer_privileges<'me, 'info>(
    accounts: TransferCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_checked_verify_account_privileges<'me, 'info>(
    accounts: TransferCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_checked_verify_writable_privileges(accounts)?;
    transfer_checked_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const APPROVE_CHECKED_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ApproveCheckedAccounts<'me, 'info> {
    ///The token account to approve spending of
    pub token_account: &'me AccountInfo<'info>,
    ///tokenAccount's mint
    pub mint: &'me AccountInfo<'info>,
    ///The delegate to approve spending to
    pub delegate: &'me AccountInfo<'info>,
    ///tokenAccount's authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveCheckedKeys {
    ///The token account to approve spending of
    pub token_account: Pubkey,
    ///tokenAccount's mint
    pub mint: Pubkey,
    ///The delegate to approve spending to
    pub delegate: Pubkey,
    ///tokenAccount's authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<ApproveCheckedAccounts<'_, '_>> for ApproveCheckedKeys {
    fn from(accounts: ApproveCheckedAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            mint: *accounts.mint.key,
            delegate: *accounts.delegate.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<ApproveCheckedKeys> for [AccountMeta; APPROVE_CHECKED_IX_ACCOUNTS_LEN] {
    fn from(keys: ApproveCheckedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.delegate,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; APPROVE_CHECKED_IX_ACCOUNTS_LEN]> for ApproveCheckedKeys {
    fn from(pubkeys: [Pubkey; APPROVE_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            mint: pubkeys[1],
            delegate: pubkeys[2],
            authority: pubkeys[3],
        }
    }
}
impl<'info> From<ApproveCheckedAccounts<'_, 'info>>
    for [AccountInfo<'info>; APPROVE_CHECKED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ApproveCheckedAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.mint.clone(),
            accounts.delegate.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; APPROVE_CHECKED_IX_ACCOUNTS_LEN]>
    for ApproveCheckedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; APPROVE_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            mint: &arr[1],
            delegate: &arr[2],
            authority: &arr[3],
        }
    }
}
pub const APPROVE_CHECKED_IX_DISCM: u8 = 13u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApproveCheckedIxArgs {
    pub args: CheckedOpArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveCheckedIxData(pub ApproveCheckedIxArgs);
impl From<ApproveCheckedIxArgs> for ApproveCheckedIxData {
    fn from(args: ApproveCheckedIxArgs) -> Self {
        Self(args)
    }
}
impl ApproveCheckedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != APPROVE_CHECKED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    APPROVE_CHECKED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ApproveCheckedIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[APPROVE_CHECKED_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn approve_checked_ix(
    keys: ApproveCheckedKeys,
    args: ApproveCheckedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; APPROVE_CHECKED_IX_ACCOUNTS_LEN] = keys.into();
    let data: ApproveCheckedIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn approve_checked_invoke<'info>(
    accounts: ApproveCheckedAccounts<'_, 'info>,
    args: ApproveCheckedIxArgs,
) -> ProgramResult {
    let keys: ApproveCheckedKeys = accounts.into();
    let ix = approve_checked_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; APPROVE_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn approve_checked_invoke_signed<'info>(
    accounts: ApproveCheckedAccounts<'_, 'info>,
    args: ApproveCheckedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ApproveCheckedKeys = accounts.into();
    let ix = approve_checked_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; APPROVE_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn approve_checked_verify_account_keys(
    accounts: ApproveCheckedAccounts<'_, '_>,
    keys: ApproveCheckedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.delegate.key, &keys.delegate),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn approve_checked_verify_writable_privileges<'me, 'info>(
    accounts: ApproveCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn approve_checked_verify_signer_privileges<'me, 'info>(
    accounts: ApproveCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn approve_checked_verify_account_privileges<'me, 'info>(
    accounts: ApproveCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    approve_checked_verify_writable_privileges(accounts)?;
    approve_checked_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MINT_TO_CHECKED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MintToCheckedAccounts<'me, 'info> {
    ///tokenAccount's mint
    pub mint: &'me AccountInfo<'info>,
    ///The token account to mint tokens to
    pub token_account: &'me AccountInfo<'info>,
    ///The mint authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct MintToCheckedKeys {
    ///tokenAccount's mint
    pub mint: Pubkey,
    ///The token account to mint tokens to
    pub token_account: Pubkey,
    ///The mint authority. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<MintToCheckedAccounts<'_, '_>> for MintToCheckedKeys {
    fn from(accounts: MintToCheckedAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            token_account: *accounts.token_account.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<MintToCheckedKeys> for [AccountMeta; MINT_TO_CHECKED_IX_ACCOUNTS_LEN] {
    fn from(keys: MintToCheckedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MINT_TO_CHECKED_IX_ACCOUNTS_LEN]> for MintToCheckedKeys {
    fn from(pubkeys: [Pubkey; MINT_TO_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            token_account: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<MintToCheckedAccounts<'_, 'info>>
    for [AccountInfo<'info>; MINT_TO_CHECKED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MintToCheckedAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.token_account.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MINT_TO_CHECKED_IX_ACCOUNTS_LEN]>
    for MintToCheckedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MINT_TO_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            token_account: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const MINT_TO_CHECKED_IX_DISCM: u8 = 14u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MintToCheckedIxArgs {
    pub args: CheckedOpArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintToCheckedIxData(pub MintToCheckedIxArgs);
impl From<MintToCheckedIxArgs> for MintToCheckedIxData {
    fn from(args: MintToCheckedIxArgs) -> Self {
        Self(args)
    }
}
impl MintToCheckedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != MINT_TO_CHECKED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MINT_TO_CHECKED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MintToCheckedIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MINT_TO_CHECKED_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mint_to_checked_ix(
    keys: MintToCheckedKeys,
    args: MintToCheckedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MINT_TO_CHECKED_IX_ACCOUNTS_LEN] = keys.into();
    let data: MintToCheckedIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_to_checked_invoke<'info>(
    accounts: MintToCheckedAccounts<'_, 'info>,
    args: MintToCheckedIxArgs,
) -> ProgramResult {
    let keys: MintToCheckedKeys = accounts.into();
    let ix = mint_to_checked_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; MINT_TO_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn mint_to_checked_invoke_signed<'info>(
    accounts: MintToCheckedAccounts<'_, 'info>,
    args: MintToCheckedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MintToCheckedKeys = accounts.into();
    let ix = mint_to_checked_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; MINT_TO_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn mint_to_checked_verify_account_keys(
    accounts: MintToCheckedAccounts<'_, '_>,
    keys: MintToCheckedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.mint.key, &keys.mint),
        (accounts.token_account.key, &keys.token_account),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn mint_to_checked_verify_writable_privileges<'me, 'info>(
    accounts: MintToCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.mint, accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mint_to_checked_verify_signer_privileges<'me, 'info>(
    accounts: MintToCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mint_to_checked_verify_account_privileges<'me, 'info>(
    accounts: MintToCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mint_to_checked_verify_writable_privileges(accounts)?;
    mint_to_checked_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BURN_CHECKED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct BurnCheckedAccounts<'me, 'info> {
    ///The token account to burn tokens from
    pub token_account: &'me AccountInfo<'info>,
    ///tokenAccount's mint
    pub mint: &'me AccountInfo<'info>,
    ///tokenAccount's authority/delegate. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct BurnCheckedKeys {
    ///The token account to burn tokens from
    pub token_account: Pubkey,
    ///tokenAccount's mint
    pub mint: Pubkey,
    ///tokenAccount's authority/delegate. If multisig, this account is not a signer and the signing signatories must follow.
    pub authority: Pubkey,
}
impl From<BurnCheckedAccounts<'_, '_>> for BurnCheckedKeys {
    fn from(accounts: BurnCheckedAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            mint: *accounts.mint.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<BurnCheckedKeys> for [AccountMeta; BURN_CHECKED_IX_ACCOUNTS_LEN] {
    fn from(keys: BurnCheckedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BURN_CHECKED_IX_ACCOUNTS_LEN]> for BurnCheckedKeys {
    fn from(pubkeys: [Pubkey; BURN_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            mint: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<BurnCheckedAccounts<'_, 'info>>
    for [AccountInfo<'info>; BURN_CHECKED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: BurnCheckedAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.mint.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BURN_CHECKED_IX_ACCOUNTS_LEN]>
    for BurnCheckedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; BURN_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            mint: &arr[1],
            authority: &arr[2],
        }
    }
}
pub const BURN_CHECKED_IX_DISCM: u8 = 15u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BurnCheckedIxArgs {
    pub args: CheckedOpArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnCheckedIxData(pub BurnCheckedIxArgs);
impl From<BurnCheckedIxArgs> for BurnCheckedIxData {
    fn from(args: BurnCheckedIxArgs) -> Self {
        Self(args)
    }
}
impl BurnCheckedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != BURN_CHECKED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BURN_CHECKED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(BurnCheckedIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[BURN_CHECKED_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn burn_checked_ix(
    keys: BurnCheckedKeys,
    args: BurnCheckedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BURN_CHECKED_IX_ACCOUNTS_LEN] = keys.into();
    let data: BurnCheckedIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn burn_checked_invoke<'info>(
    accounts: BurnCheckedAccounts<'_, 'info>,
    args: BurnCheckedIxArgs,
) -> ProgramResult {
    let keys: BurnCheckedKeys = accounts.into();
    let ix = burn_checked_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; BURN_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn burn_checked_invoke_signed<'info>(
    accounts: BurnCheckedAccounts<'_, 'info>,
    args: BurnCheckedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BurnCheckedKeys = accounts.into();
    let ix = burn_checked_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; BURN_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn burn_checked_verify_account_keys(
    accounts: BurnCheckedAccounts<'_, '_>,
    keys: BurnCheckedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn burn_checked_verify_writable_privileges<'me, 'info>(
    accounts: BurnCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account, accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn burn_checked_verify_signer_privileges<'me, 'info>(
    accounts: BurnCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn burn_checked_verify_account_privileges<'me, 'info>(
    accounts: BurnCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    burn_checked_verify_writable_privileges(accounts)?;
    burn_checked_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccount2Accounts<'me, 'info> {
    ///The token account to initialize
    pub token_account: &'me AccountInfo<'info>,
    ///tokenAccount's mint
    pub mint: &'me AccountInfo<'info>,
    ///Rent sysvar
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccount2Keys {
    ///The token account to initialize
    pub token_account: Pubkey,
    ///tokenAccount's mint
    pub mint: Pubkey,
    ///Rent sysvar
    pub rent: Pubkey,
}
impl From<InitializeAccount2Accounts<'_, '_>> for InitializeAccount2Keys {
    fn from(accounts: InitializeAccount2Accounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            mint: *accounts.mint.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeAccount2Keys> for [AccountMeta; INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeAccount2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN]> for InitializeAccount2Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            mint: pubkeys[1],
            rent: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeAccount2Accounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccount2Accounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.mint.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN]>
    for InitializeAccount2Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            mint: &arr[1],
            rent: &arr[2],
        }
    }
}
pub const INITIALIZE_ACCOUNT2_IX_DISCM: u8 = 16u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeAccount2IxArgs {
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeAccount2IxData(pub InitializeAccount2IxArgs);
impl From<InitializeAccount2IxArgs> for InitializeAccount2IxData {
    fn from(args: InitializeAccount2IxArgs) -> Self {
        Self(args)
    }
}
impl InitializeAccount2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_ACCOUNT2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_ACCOUNT2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeAccount2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_ACCOUNT2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_account2_ix(
    keys: InitializeAccount2Keys,
    args: InitializeAccount2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeAccount2IxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_account2_invoke<'info>(
    accounts: InitializeAccount2Accounts<'_, 'info>,
    args: InitializeAccount2IxArgs,
) -> ProgramResult {
    let keys: InitializeAccount2Keys = accounts.into();
    let ix = initialize_account2_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_account2_invoke_signed<'info>(
    accounts: InitializeAccount2Accounts<'_, 'info>,
    args: InitializeAccount2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeAccount2Keys = accounts.into();
    let ix = initialize_account2_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_ACCOUNT2_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_account2_verify_account_keys(
    accounts: InitializeAccount2Accounts<'_, '_>,
    keys: InitializeAccount2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_account2_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccount2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_account2_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccount2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_account2_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SYNC_NATIVE_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct SyncNativeAccounts<'me, 'info> {
    ///The native token account to sync with its underlying lamports
    pub token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SyncNativeKeys {
    ///The native token account to sync with its underlying lamports
    pub token_account: Pubkey,
}
impl From<SyncNativeAccounts<'_, '_>> for SyncNativeKeys {
    fn from(accounts: SyncNativeAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
        }
    }
}
impl From<SyncNativeKeys> for [AccountMeta; SYNC_NATIVE_IX_ACCOUNTS_LEN] {
    fn from(keys: SyncNativeKeys) -> Self {
        [AccountMeta {
            pubkey: keys.token_account,
            is_signer: false,
            is_writable: true,
        }]
    }
}
impl From<[Pubkey; SYNC_NATIVE_IX_ACCOUNTS_LEN]> for SyncNativeKeys {
    fn from(pubkeys: [Pubkey; SYNC_NATIVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
        }
    }
}
impl<'info> From<SyncNativeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SYNC_NATIVE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SyncNativeAccounts<'_, 'info>) -> Self {
        [accounts.token_account.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SYNC_NATIVE_IX_ACCOUNTS_LEN]>
    for SyncNativeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SYNC_NATIVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
        }
    }
}
pub const SYNC_NATIVE_IX_DISCM: u8 = 17u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SyncNativeIxData;
impl SyncNativeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SYNC_NATIVE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SYNC_NATIVE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SYNC_NATIVE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sync_native_ix(keys: SyncNativeKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SYNC_NATIVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: SyncNativeIxData.try_to_vec()?,
    })
}
pub fn sync_native_invoke<'info>(accounts: SyncNativeAccounts<'_, 'info>) -> ProgramResult {
    let keys: SyncNativeKeys = accounts.into();
    let ix = sync_native_ix(keys)?;
    let account_info: [AccountInfo<'info>; SYNC_NATIVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn sync_native_invoke_signed<'info>(
    accounts: SyncNativeAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SyncNativeKeys = accounts.into();
    let ix = sync_native_ix(keys)?;
    let account_info: [AccountInfo<'info>; SYNC_NATIVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn sync_native_verify_account_keys(
    accounts: SyncNativeAccounts<'_, '_>,
    keys: SyncNativeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.token_account.key, &keys.token_account)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn sync_native_verify_writable_privileges<'me, 'info>(
    accounts: SyncNativeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sync_native_verify_account_privileges<'me, 'info>(
    accounts: SyncNativeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sync_native_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccount3Accounts<'me, 'info> {
    ///The token account to initialize
    pub token_account: &'me AccountInfo<'info>,
    ///tokenAccount's mint
    pub mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccount3Keys {
    ///The token account to initialize
    pub token_account: Pubkey,
    ///tokenAccount's mint
    pub mint: Pubkey,
}
impl From<InitializeAccount3Accounts<'_, '_>> for InitializeAccount3Keys {
    fn from(accounts: InitializeAccount3Accounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            mint: *accounts.mint.key,
        }
    }
}
impl From<InitializeAccount3Keys> for [AccountMeta; INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeAccount3Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN]> for InitializeAccount3Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            mint: pubkeys[1],
        }
    }
}
impl<'info> From<InitializeAccount3Accounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccount3Accounts<'_, 'info>) -> Self {
        [accounts.token_account.clone(), accounts.mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN]>
    for InitializeAccount3Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            mint: &arr[1],
        }
    }
}
pub const INITIALIZE_ACCOUNT3_IX_DISCM: u8 = 18u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeAccount3IxArgs {
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeAccount3IxData(pub InitializeAccount3IxArgs);
impl From<InitializeAccount3IxArgs> for InitializeAccount3IxData {
    fn from(args: InitializeAccount3IxArgs) -> Self {
        Self(args)
    }
}
impl InitializeAccount3IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_ACCOUNT3_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_ACCOUNT3_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeAccount3IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_ACCOUNT3_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_account3_ix(
    keys: InitializeAccount3Keys,
    args: InitializeAccount3IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeAccount3IxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_account3_invoke<'info>(
    accounts: InitializeAccount3Accounts<'_, 'info>,
    args: InitializeAccount3IxArgs,
) -> ProgramResult {
    let keys: InitializeAccount3Keys = accounts.into();
    let ix = initialize_account3_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_account3_invoke_signed<'info>(
    accounts: InitializeAccount3Accounts<'_, 'info>,
    args: InitializeAccount3IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeAccount3Keys = accounts.into();
    let ix = initialize_account3_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_ACCOUNT3_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_account3_verify_account_keys(
    accounts: InitializeAccount3Accounts<'_, '_>,
    keys: InitializeAccount3Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_account.key, &keys.token_account),
        (accounts.mint.key, &keys.mint),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_account3_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccount3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_account3_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccount3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_account3_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct InitializeMultisig2Accounts<'me, 'info> {
    ///The multisignature account to initialize. The signer accounts suffix slice follows. Length must equal to N where 1 <= N <= 11
    pub multisig: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeMultisig2Keys {
    ///The multisignature account to initialize. The signer accounts suffix slice follows. Length must equal to N where 1 <= N <= 11
    pub multisig: Pubkey,
}
impl From<InitializeMultisig2Accounts<'_, '_>> for InitializeMultisig2Keys {
    fn from(accounts: InitializeMultisig2Accounts) -> Self {
        Self {
            multisig: *accounts.multisig.key,
        }
    }
}
impl From<InitializeMultisig2Keys> for [AccountMeta; INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeMultisig2Keys) -> Self {
        [AccountMeta {
            pubkey: keys.multisig,
            is_signer: false,
            is_writable: true,
        }]
    }
}
impl From<[Pubkey; INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN]> for InitializeMultisig2Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            multisig: pubkeys[0],
        }
    }
}
impl<'info> From<InitializeMultisig2Accounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeMultisig2Accounts<'_, 'info>) -> Self {
        [accounts.multisig.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN]>
    for InitializeMultisig2Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN]) -> Self {
        Self { multisig: &arr[0] }
    }
}
pub const INITIALIZE_MULTISIG2_IX_DISCM: u8 = 19u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeMultisig2IxArgs {
    pub m: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMultisig2IxData(pub InitializeMultisig2IxArgs);
impl From<InitializeMultisig2IxArgs> for InitializeMultisig2IxData {
    fn from(args: InitializeMultisig2IxArgs) -> Self {
        Self(args)
    }
}
impl InitializeMultisig2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_MULTISIG2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_MULTISIG2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeMultisig2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_MULTISIG2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_multisig2_ix(
    keys: InitializeMultisig2Keys,
    args: InitializeMultisig2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeMultisig2IxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_multisig2_invoke<'info>(
    accounts: InitializeMultisig2Accounts<'_, 'info>,
    args: InitializeMultisig2IxArgs,
) -> ProgramResult {
    let keys: InitializeMultisig2Keys = accounts.into();
    let ix = initialize_multisig2_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_multisig2_invoke_signed<'info>(
    accounts: InitializeMultisig2Accounts<'_, 'info>,
    args: InitializeMultisig2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeMultisig2Keys = accounts.into();
    let ix = initialize_multisig2_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MULTISIG2_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_multisig2_verify_account_keys(
    accounts: InitializeMultisig2Accounts<'_, '_>,
    keys: InitializeMultisig2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.multisig.key, &keys.multisig)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_multisig2_verify_writable_privileges<'me, 'info>(
    accounts: InitializeMultisig2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.multisig] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_multisig2_verify_account_privileges<'me, 'info>(
    accounts: InitializeMultisig2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_multisig2_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_MINT2_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct InitializeMint2Accounts<'me, 'info> {
    ///The mint to initialize
    pub mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeMint2Keys {
    ///The mint to initialize
    pub mint: Pubkey,
}
impl From<InitializeMint2Accounts<'_, '_>> for InitializeMint2Keys {
    fn from(accounts: InitializeMint2Accounts) -> Self {
        Self {
            mint: *accounts.mint.key,
        }
    }
}
impl From<InitializeMint2Keys> for [AccountMeta; INITIALIZE_MINT2_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeMint2Keys) -> Self {
        [AccountMeta {
            pubkey: keys.mint,
            is_signer: false,
            is_writable: true,
        }]
    }
}
impl From<[Pubkey; INITIALIZE_MINT2_IX_ACCOUNTS_LEN]> for InitializeMint2Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_MINT2_IX_ACCOUNTS_LEN]) -> Self {
        Self { mint: pubkeys[0] }
    }
}
impl<'info> From<InitializeMint2Accounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_MINT2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeMint2Accounts<'_, 'info>) -> Self {
        [accounts.mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_MINT2_IX_ACCOUNTS_LEN]>
    for InitializeMint2Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_MINT2_IX_ACCOUNTS_LEN]) -> Self {
        Self { mint: &arr[0] }
    }
}
pub const INITIALIZE_MINT2_IX_DISCM: u8 = 20u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeMint2IxArgs {
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMint2IxData(pub InitializeMint2IxArgs);
impl From<InitializeMint2IxArgs> for InitializeMint2IxData {
    fn from(args: InitializeMint2IxArgs) -> Self {
        Self(args)
    }
}
impl InitializeMint2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_MINT2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_MINT2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeMint2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_MINT2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_mint2_ix(
    keys: InitializeMint2Keys,
    args: InitializeMint2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_MINT2_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeMint2IxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_mint2_invoke<'info>(
    accounts: InitializeMint2Accounts<'_, 'info>,
    args: InitializeMint2IxArgs,
) -> ProgramResult {
    let keys: InitializeMint2Keys = accounts.into();
    let ix = initialize_mint2_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MINT2_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_mint2_invoke_signed<'info>(
    accounts: InitializeMint2Accounts<'_, 'info>,
    args: InitializeMint2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeMint2Keys = accounts.into();
    let ix = initialize_mint2_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MINT2_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_mint2_verify_account_keys(
    accounts: InitializeMint2Accounts<'_, '_>,
    keys: InitializeMint2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.mint.key, &keys.mint)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_mint2_verify_writable_privileges<'me, 'info>(
    accounts: InitializeMint2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_mint2_verify_account_privileges<'me, 'info>(
    accounts: InitializeMint2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_mint2_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct GetTokenAccountDataSizeAccounts<'me, 'info> {
    ///The mint to calculate for
    pub mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct GetTokenAccountDataSizeKeys {
    ///The mint to calculate for
    pub mint: Pubkey,
}
impl From<GetTokenAccountDataSizeAccounts<'_, '_>> for GetTokenAccountDataSizeKeys {
    fn from(accounts: GetTokenAccountDataSizeAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
        }
    }
}
impl From<GetTokenAccountDataSizeKeys>
    for [AccountMeta; GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(keys: GetTokenAccountDataSizeKeys) -> Self {
        [AccountMeta {
            pubkey: keys.mint,
            is_signer: false,
            is_writable: false,
        }]
    }
}
impl From<[Pubkey; GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN]> for GetTokenAccountDataSizeKeys {
    fn from(pubkeys: [Pubkey; GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self { mint: pubkeys[0] }
    }
}
impl<'info> From<GetTokenAccountDataSizeAccounts<'_, 'info>>
    for [AccountInfo<'info>; GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: GetTokenAccountDataSizeAccounts<'_, 'info>) -> Self {
        [accounts.mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN]>
    for GetTokenAccountDataSizeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self { mint: &arr[0] }
    }
}
pub const GET_TOKEN_ACCOUNT_DATA_SIZE_IX_DISCM: u8 = 21u8;
#[derive(Clone, Debug, PartialEq)]
pub struct GetTokenAccountDataSizeIxData;
impl GetTokenAccountDataSizeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != GET_TOKEN_ACCOUNT_DATA_SIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    GET_TOKEN_ACCOUNT_DATA_SIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[GET_TOKEN_ACCOUNT_DATA_SIZE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_token_account_data_size_ix(
    keys: GetTokenAccountDataSizeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: GetTokenAccountDataSizeIxData.try_to_vec()?,
    })
}
pub fn get_token_account_data_size_invoke<'info>(
    accounts: GetTokenAccountDataSizeAccounts<'_, 'info>,
) -> ProgramResult {
    let keys: GetTokenAccountDataSizeKeys = accounts.into();
    let ix = get_token_account_data_size_ix(keys)?;
    let account_info: [AccountInfo<'info>; GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn get_token_account_data_size_invoke_signed<'info>(
    accounts: GetTokenAccountDataSizeAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetTokenAccountDataSizeKeys = accounts.into();
    let ix = get_token_account_data_size_ix(keys)?;
    let account_info: [AccountInfo<'info>; GET_TOKEN_ACCOUNT_DATA_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn get_token_account_data_size_verify_account_keys(
    accounts: GetTokenAccountDataSizeAccounts<'_, '_>,
    keys: GetTokenAccountDataSizeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.mint.key, &keys.mint)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub const INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct InitializeImmutableOwnerAccounts<'me, 'info> {
    ///The token account to initialize
    pub token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeImmutableOwnerKeys {
    ///The token account to initialize
    pub token_account: Pubkey,
}
impl From<InitializeImmutableOwnerAccounts<'_, '_>> for InitializeImmutableOwnerKeys {
    fn from(accounts: InitializeImmutableOwnerAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
        }
    }
}
impl From<InitializeImmutableOwnerKeys>
    for [AccountMeta; INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN]
{
    fn from(keys: InitializeImmutableOwnerKeys) -> Self {
        [AccountMeta {
            pubkey: keys.token_account,
            is_signer: false,
            is_writable: true,
        }]
    }
}
impl From<[Pubkey; INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN]> for InitializeImmutableOwnerKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
        }
    }
}
impl<'info> From<InitializeImmutableOwnerAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeImmutableOwnerAccounts<'_, 'info>) -> Self {
        [accounts.token_account.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN]>
    for InitializeImmutableOwnerAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
        }
    }
}
pub const INITIALIZE_IMMUTABLE_OWNER_IX_DISCM: u8 = 22u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeImmutableOwnerIxData;
impl InitializeImmutableOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_IMMUTABLE_OWNER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IMMUTABLE_OWNER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_IMMUTABLE_OWNER_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_immutable_owner_ix(
    keys: InitializeImmutableOwnerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: InitializeImmutableOwnerIxData.try_to_vec()?,
    })
}
pub fn initialize_immutable_owner_invoke<'info>(
    accounts: InitializeImmutableOwnerAccounts<'_, 'info>,
) -> ProgramResult {
    let keys: InitializeImmutableOwnerKeys = accounts.into();
    let ix = initialize_immutable_owner_ix(keys)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_immutable_owner_invoke_signed<'info>(
    accounts: InitializeImmutableOwnerAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeImmutableOwnerKeys = accounts.into();
    let ix = initialize_immutable_owner_ix(keys)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_IMMUTABLE_OWNER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_immutable_owner_verify_account_keys(
    accounts: InitializeImmutableOwnerAccounts<'_, '_>,
    keys: InitializeImmutableOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.token_account.key, &keys.token_account)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_immutable_owner_verify_writable_privileges<'me, 'info>(
    accounts: InitializeImmutableOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_immutable_owner_verify_account_privileges<'me, 'info>(
    accounts: InitializeImmutableOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_immutable_owner_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct AmountToUiAmountAccounts<'me, 'info> {
    ///The mint to calculate for
    pub mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AmountToUiAmountKeys {
    ///The mint to calculate for
    pub mint: Pubkey,
}
impl From<AmountToUiAmountAccounts<'_, '_>> for AmountToUiAmountKeys {
    fn from(accounts: AmountToUiAmountAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
        }
    }
}
impl From<AmountToUiAmountKeys> for [AccountMeta; AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: AmountToUiAmountKeys) -> Self {
        [AccountMeta {
            pubkey: keys.mint,
            is_signer: false,
            is_writable: false,
        }]
    }
}
impl From<[Pubkey; AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN]> for AmountToUiAmountKeys {
    fn from(pubkeys: [Pubkey; AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self { mint: pubkeys[0] }
    }
}
impl<'info> From<AmountToUiAmountAccounts<'_, 'info>>
    for [AccountInfo<'info>; AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AmountToUiAmountAccounts<'_, 'info>) -> Self {
        [accounts.mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN]>
    for AmountToUiAmountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self { mint: &arr[0] }
    }
}
pub const AMOUNT_TO_UI_AMOUNT_IX_DISCM: u8 = 23u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AmountToUiAmountIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmountToUiAmountIxData(pub AmountToUiAmountIxArgs);
impl From<AmountToUiAmountIxArgs> for AmountToUiAmountIxData {
    fn from(args: AmountToUiAmountIxArgs) -> Self {
        Self(args)
    }
}
impl AmountToUiAmountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != AMOUNT_TO_UI_AMOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    AMOUNT_TO_UI_AMOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AmountToUiAmountIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[AMOUNT_TO_UI_AMOUNT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn amount_to_ui_amount_ix(
    keys: AmountToUiAmountKeys,
    args: AmountToUiAmountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: AmountToUiAmountIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn amount_to_ui_amount_invoke<'info>(
    accounts: AmountToUiAmountAccounts<'_, 'info>,
    args: AmountToUiAmountIxArgs,
) -> ProgramResult {
    let keys: AmountToUiAmountKeys = accounts.into();
    let ix = amount_to_ui_amount_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn amount_to_ui_amount_invoke_signed<'info>(
    accounts: AmountToUiAmountAccounts<'_, 'info>,
    args: AmountToUiAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AmountToUiAmountKeys = accounts.into();
    let ix = amount_to_ui_amount_ix(keys, args)?;
    let account_info: [AccountInfo<'info>; AMOUNT_TO_UI_AMOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn amount_to_ui_amount_verify_account_keys(
    accounts: AmountToUiAmountAccounts<'_, '_>,
    keys: AmountToUiAmountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.mint.key, &keys.mint)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
