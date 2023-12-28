use crate::ReadonlyMintAccount;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_readonly_account::ReadonlyAccountData;
use spl_token_interface::CheckedOpArgs;

fn mint_decimals_checked<M: ReadonlyAccountData>(mint: M) -> Result<u8, ProgramError> {
    if !mint.mint_data_is_valid() || !mint.mint_is_initialized() {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(mint.mint_decimals())
}

macro_rules! impl_decimal_agnostic {
    (
        $ix_fn: ident,
        $invoke_fn: ident,
        $invoke_signed_fn: ident,
        $ix_fn_with_program_id: ident,
        $invoke_fn_with_program_id: ident,
        $invoke_signed_fn_with_program_id: ident,
        $multisig_ix_fn: ident,
        $multisig_invoke_fn: ident,
        $multisig_invoke_signed_fn: ident,
        $multisig_ix_fn_with_program_id: ident,
        $multisig_invoke_fn_with_program_id: ident,
        $multisig_invoke_signed_fn_with_program_id: ident,

        $ix_with_program_id: ident,
        $accounts: ident,
        $keys: ident,
        $ix_args: ident,
        $accounts_len: ident,
    ) => {
        use spl_token_interface::{$accounts, $accounts_len, $ix_args, $ix_with_program_id, $keys};

        pub fn $ix_fn<M: ReadonlyAccountData>(
            keys: $keys,
            amount: u64,
            mint: M,
        ) -> Result<Instruction, ProgramError> {
            $ix_fn_with_program_id(spl_token_interface::ID, keys, amount, mint)
        }

        pub fn $invoke_fn(accounts: $accounts, amount: u64) -> ProgramResult {
            $invoke_fn_with_program_id(spl_token_interface::ID, accounts, amount)
        }

        pub fn $invoke_signed_fn(
            accounts: $accounts,
            amount: u64,
            seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            $invoke_signed_fn_with_program_id(spl_token_interface::ID, accounts, amount, seeds)
        }

        pub fn $ix_fn_with_program_id<M: ReadonlyAccountData>(
            program_id: Pubkey,
            keys: $keys,
            amount: u64,
            mint: M,
        ) -> Result<Instruction, ProgramError> {
            let decimals = mint_decimals_checked(mint)?;
            Ok($ix_with_program_id(
                program_id,
                keys,
                $ix_args {
                    args: CheckedOpArgs { amount, decimals },
                },
            )?)
        }

        pub fn $invoke_fn_with_program_id(
            program_id: Pubkey,
            accounts: $accounts,
            amount: u64,
        ) -> ProgramResult {
            let ix = $ix_fn_with_program_id(program_id, accounts.into(), amount, accounts.mint)?;
            let account_infos: [AccountInfo<'_>; $accounts_len] = accounts.into();
            invoke(&ix, &account_infos)
        }

        pub fn $invoke_signed_fn_with_program_id(
            program_id: Pubkey,
            accounts: $accounts,
            amount: u64,
            seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            let ix = $ix_fn_with_program_id(program_id, accounts.into(), amount, accounts.mint)?;
            let account_infos: [AccountInfo<'_>; $accounts_len] = accounts.into();
            invoke_signed(&ix, &account_infos, seeds)
        }

        pub fn $multisig_ix_fn<M: ReadonlyAccountData>(
            keys: $keys,
            amount: u64,
            mint: M,
            signatories: impl Iterator<Item = Pubkey>,
        ) -> Result<Instruction, ProgramError> {
            $multisig_ix_fn_with_program_id(
                spl_token_interface::ID,
                keys,
                amount,
                mint,
                signatories,
            )
        }

        pub fn $multisig_invoke_fn<'a, 'info>(
            accounts: $accounts<'a, 'info>,
            amount: u64,
            signatories: &'a [AccountInfo<'info>],
        ) -> ProgramResult {
            $multisig_invoke_fn_with_program_id(
                spl_token_interface::ID,
                accounts,
                amount,
                signatories,
            )
        }

        pub fn $multisig_invoke_signed_fn<'a, 'info>(
            accounts: $accounts<'a, 'info>,
            amount: u64,
            signatories: &'a [AccountInfo<'info>],
            seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            $multisig_invoke_signed_fn_with_program_id(
                spl_token_interface::ID,
                accounts,
                amount,
                signatories,
                seeds,
            )
        }

        pub fn $multisig_ix_fn_with_program_id<M: ReadonlyAccountData>(
            program_id: Pubkey,
            keys: $keys,
            amount: u64,
            mint: M,
            signatories: impl Iterator<Item = Pubkey>,
        ) -> Result<Instruction, ProgramError> {
            let mut ix = $ix_fn_with_program_id(program_id, keys, amount, mint)?;
            ix.accounts.last_mut().unwrap().is_signer = false;
            ix.accounts.extend(signatories.map(|pubkey| AccountMeta {
                pubkey,
                is_signer: true,
                is_writable: false,
            }));
            Ok(ix)
        }

        pub fn $multisig_invoke_fn_with_program_id<'a, 'info>(
            program_id: Pubkey,
            accounts: $accounts<'a, 'info>,
            amount: u64,
            signatories: &'a [AccountInfo<'info>],
        ) -> ProgramResult {
            let ix = $multisig_ix_fn_with_program_id(
                program_id,
                accounts.into(),
                amount,
                accounts.mint,
                signatories.iter().map(|a| *a.key),
            )?;
            let mut accounts = Vec::from(Into::<[AccountInfo; $accounts_len]>::into(accounts));
            accounts.extend(signatories.iter().cloned());
            invoke(&ix, &accounts)
        }

        pub fn $multisig_invoke_signed_fn_with_program_id<'a, 'info>(
            program_id: Pubkey,
            accounts: $accounts<'a, 'info>,
            amount: u64,
            signatories: &'a [AccountInfo<'info>],
            seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            let ix = $multisig_ix_fn_with_program_id(
                program_id,
                accounts.into(),
                amount,
                accounts.mint,
                signatories.iter().map(|a| *a.key),
            )?;
            let mut accounts = Vec::from(Into::<[AccountInfo; $accounts_len]>::into(accounts));
            accounts.extend(signatories.iter().cloned());
            invoke_signed(&ix, &accounts, seeds)
        }
    };
}

impl_decimal_agnostic!(
    transfer_checked_decimal_agnostic_ix,
    transfer_checked_decimal_agnostic_invoke,
    transfer_checked_decimal_agnostic_invoke_signed,
    transfer_checked_decimal_agnostic_ix_with_program_id,
    transfer_checked_decimal_agnostic_invoke_with_program_id,
    transfer_checked_decimal_agnostic_invoke_signed_with_program_id,
    transfer_checked_decimal_agnostic_multisig_ix,
    transfer_checked_decimal_agnostic_multisig_invoke,
    transfer_checked_decimal_agnostic_multisig_invoke_signed,
    transfer_checked_decimal_agnostic_multisig_ix_with_program_id,
    transfer_checked_decimal_agnostic_multisig_invoke_with_program_id,
    transfer_checked_decimal_agnostic_multisig_invoke_signed_with_program_id,
    transfer_checked_ix_with_program_id,
    TransferCheckedAccounts,
    TransferCheckedKeys,
    TransferCheckedIxArgs,
    TRANSFER_CHECKED_IX_ACCOUNTS_LEN,
);

impl_decimal_agnostic!(
    burn_checked_decimal_agnostic_ix,
    burn_checked_decimal_agnostic_invoke,
    burn_checked_decimal_agnostic_invoke_signed,
    burn_checked_decimal_agnostic_ix_with_program_id,
    burn_checked_decimal_agnostic_invoke_with_program_id,
    burn_checked_decimal_agnostic_invoke_signed_with_program_id,
    burn_checked_decimal_agnostic_multisig_ix,
    burn_checked_decimal_agnostic_multisig_invoke,
    burn_checked_decimal_agnostic_multisig_invoke_signed,
    burn_checked_decimal_agnostic_multisig_ix_with_program_id,
    burn_checked_decimal_agnostic_multisig_invoke_with_program_id,
    burn_checked_decimal_agnostic_multisig_invoke_signed_with_program_id,
    burn_checked_ix_with_program_id,
    BurnCheckedAccounts,
    BurnCheckedKeys,
    BurnCheckedIxArgs,
    BURN_CHECKED_IX_ACCOUNTS_LEN,
);

impl_decimal_agnostic!(
    approve_checked_decimal_agnostic_ix,
    approve_checked_decimal_agnostic_invoke,
    approve_checked_decimal_agnostic_invoke_signed,
    approve_checked_decimal_agnostic_ix_with_program_id,
    approve_checked_decimal_agnostic_invoke_with_program_id,
    approve_checked_decimal_agnostic_invoke_signed_with_program_id,
    approve_checked_decimal_agnostic_multisig_ix,
    approve_checked_decimal_agnostic_multisig_invoke,
    approve_checked_decimal_agnostic_multisig_invoke_signed,
    approve_checked_decimal_agnostic_multisig_ix_with_program_id,
    approve_checked_decimal_agnostic_multisig_invoke_with_program_id,
    approve_checked_decimal_agnostic_multisig_invoke_signed_with_program_id,
    approve_checked_ix_with_program_id,
    ApproveCheckedAccounts,
    ApproveCheckedKeys,
    ApproveCheckedIxArgs,
    APPROVE_CHECKED_IX_ACCOUNTS_LEN,
);

impl_decimal_agnostic!(
    mint_to_checked_decimal_agnostic_ix,
    mint_to_checked_decimal_agnostic_invoke,
    mint_to_checked_decimal_agnostic_invoke_signed,
    mint_to_checked_decimal_agnostic_ix_with_program_id,
    mint_to_checked_decimal_agnostic_invoke_with_program_id,
    mint_to_checked_decimal_agnostic_invoke_signed_with_program_id,
    mint_to_checked_decimal_agnostic_multisig_ix,
    mint_to_checked_decimal_agnostic_multisig_invoke,
    mint_to_checked_decimal_agnostic_multisig_invoke_signed,
    mint_to_checked_decimal_agnostic_multisig_ix_with_program_id,
    mint_to_checked_decimal_agnostic_multisig_invoke_with_program_id,
    mint_to_checked_decimal_agnostic_multisig_invoke_signed_with_program_id,
    mint_to_checked_ix_with_program_id,
    MintToCheckedAccounts,
    MintToCheckedKeys,
    MintToCheckedIxArgs,
    MINT_TO_CHECKED_IX_ACCOUNTS_LEN,
);
