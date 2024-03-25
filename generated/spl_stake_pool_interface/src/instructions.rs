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
pub enum SplStakePoolProgramIx {
    Initialize(InitializeIxArgs),
}
impl SplStakePoolProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            INITIALIZE_IX_DISCM => Ok(Self::Initialize(InitializeIxArgs::deserialize(
                &mut reader,
            )?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Initialize(args) => {
                writer.write_all(&[INITIALIZE_IX_DISCM])?;
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
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    ///New StakePool to create
    pub stake_pool: &'me AccountInfo<'info>,
    ///Manager
    pub manager: &'me AccountInfo<'info>,
    ///Staker
    pub staker: &'me AccountInfo<'info>,
    ///Stake pool withdraw authority
    pub withdraw_authority: &'me AccountInfo<'info>,
    ///Uninitialized validator stake list storage account
    pub validator_list: &'me AccountInfo<'info>,
    ///Reserve stake account must be initialized, have zero balance, and staker / withdrawer authority set to pool withdraw authority
    pub reserve_stake: &'me AccountInfo<'info>,
    ///Pool token mint. Must have zero supply, owned by withdraw authority.
    pub pool_token_mint: &'me AccountInfo<'info>,
    ///Pool account to deposit the generated fee for manager.
    pub manager_fee_account: &'me AccountInfo<'info>,
    ///Token program id. Optional deposit authority account follows. If omitted, anyone can deposit into the pool.
    pub token_program_id: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeKeys {
    ///New StakePool to create
    pub stake_pool: Pubkey,
    ///Manager
    pub manager: Pubkey,
    ///Staker
    pub staker: Pubkey,
    ///Stake pool withdraw authority
    pub withdraw_authority: Pubkey,
    ///Uninitialized validator stake list storage account
    pub validator_list: Pubkey,
    ///Reserve stake account must be initialized, have zero balance, and staker / withdrawer authority set to pool withdraw authority
    pub reserve_stake: Pubkey,
    ///Pool token mint. Must have zero supply, owned by withdraw authority.
    pub pool_token_mint: Pubkey,
    ///Pool account to deposit the generated fee for manager.
    pub manager_fee_account: Pubkey,
    ///Token program id. Optional deposit authority account follows. If omitted, anyone can deposit into the pool.
    pub token_program_id: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            manager: *accounts.manager.key,
            staker: *accounts.staker.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            validator_list: *accounts.validator_list.key,
            reserve_stake: *accounts.reserve_stake.key,
            pool_token_mint: *accounts.pool_token_mint.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            token_program_id: *accounts.token_program_id.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staker,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_stake,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_id,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            manager: pubkeys[1],
            staker: pubkeys[2],
            withdraw_authority: pubkeys[3],
            validator_list: pubkeys[4],
            reserve_stake: pubkeys[5],
            pool_token_mint: pubkeys[6],
            manager_fee_account: pubkeys[7],
            token_program_id: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.manager.clone(),
            accounts.staker.clone(),
            accounts.withdraw_authority.clone(),
            accounts.validator_list.clone(),
            accounts.reserve_stake.clone(),
            accounts.pool_token_mint.clone(),
            accounts.manager_fee_account.clone(),
            accounts.token_program_id.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            manager: &arr[1],
            staker: &arr[2],
            withdraw_authority: &arr[3],
            validator_list: &arr[4],
            reserve_stake: &arr[5],
            pool_token_mint: &arr[6],
            manager_fee_account: &arr[7],
            token_program_id: &arr[8],
        }
    }
}
pub const INITIALIZE_IX_DISCM: u8 = 0u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub fee: Fee,
    pub withdrawal_fee: Fee,
    pub deposit_fee: Fee,
    pub referral_fee: u8,
    pub max_validators: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_ix(keys: InitializeKeys, args: InitializeIxArgs) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    initialize_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.manager.key, &keys.manager),
        (accounts.staker.key, &keys.staker),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.reserve_stake.key, &keys.reserve_stake),
        (accounts.pool_token_mint.key, &keys.pool_token_mint),
        (accounts.manager_fee_account.key, &keys.manager_fee_account),
        (accounts.token_program_id.key, &keys.token_program_id),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake_pool, accounts.validator_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
