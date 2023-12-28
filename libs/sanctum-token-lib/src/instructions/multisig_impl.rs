macro_rules! multisig_impl {
    (
        $ix_fn: ident,
        $invoke_fn: ident,
        $invoke_signed_fn: ident,
        $ix_fn_with_program_id: ident,
        $invoke_fn_with_program_id: ident,
        $invoke_signed_fn_with_program_id: ident,

        $ix_with_program_id: ident,
        $accounts: ident,
        $keys: ident,
        $ix_args: ident,
        $accounts_len: ident,
    ) => {
        use solana_program::{
            account_info::AccountInfo,
            entrypoint::ProgramResult,
            instruction::{AccountMeta, Instruction},
            program::{invoke, invoke_signed},
            pubkey::Pubkey,
        };
        use spl_token_interface::{$accounts, $accounts_len, $ix_args, $ix_with_program_id, $keys};

        pub fn $ix_fn(
            keys: $keys,
            args: $ix_args,
            signatories: impl Iterator<Item = Pubkey>,
        ) -> std::io::Result<Instruction> {
            $ix_fn_with_program_id(spl_token_interface::ID, keys, args, signatories)
        }

        pub fn $invoke_fn<'a, 'info>(
            accounts: $accounts<'a, 'info>,
            args: $ix_args,
            signatories: &'a [AccountInfo<'info>],
        ) -> ProgramResult {
            $invoke_fn_with_program_id(spl_token_interface::ID, accounts, args, signatories)
        }

        pub fn $invoke_signed_fn<'a, 'info>(
            accounts: $accounts<'a, 'info>,
            args: $ix_args,
            signatories: &'a [AccountInfo<'info>],
            seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            $invoke_signed_fn_with_program_id(
                spl_token_interface::ID,
                accounts,
                args,
                signatories,
                seeds,
            )
        }

        pub fn $ix_fn_with_program_id(
            program_id: Pubkey,
            keys: $keys,
            args: $ix_args,
            signatories: impl Iterator<Item = Pubkey>,
        ) -> std::io::Result<Instruction> {
            let mut ix = $ix_with_program_id(program_id, keys, args)?;
            ix.accounts.last_mut().unwrap().is_signer = false;
            ix.accounts.extend(signatories.map(|pubkey| AccountMeta {
                pubkey,
                is_signer: true,
                is_writable: false,
            }));
            Ok(ix)
        }

        pub fn $invoke_fn_with_program_id<'a, 'info>(
            program_id: Pubkey,
            accounts: $accounts<'a, 'info>,
            args: $ix_args,
            signatories: &'a [AccountInfo<'info>],
        ) -> ProgramResult {
            let ix = $ix_fn_with_program_id(
                program_id,
                accounts.into(),
                args,
                signatories.iter().map(|a| *a.key),
            )?;
            let mut accounts = Vec::from(Into::<[AccountInfo; $accounts_len]>::into(accounts));
            accounts.extend(signatories.iter().cloned());
            invoke(&ix, &accounts)
        }

        pub fn $invoke_signed_fn_with_program_id<'a, 'info>(
            program_id: Pubkey,
            accounts: $accounts<'a, 'info>,
            args: $ix_args,
            signatories: &'a [AccountInfo<'info>],
            seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            let ix = $ix_fn_with_program_id(
                program_id,
                accounts.into(),
                args,
                signatories.iter().map(|a| *a.key),
            )?;
            let mut accounts = Vec::from(Into::<[AccountInfo; $accounts_len]>::into(accounts));
            accounts.extend(signatories.iter().cloned());
            invoke_signed(&ix, &accounts, seeds)
        }
    };
}

macro_rules! multisig_impl_no_ix_args {
    (
        $ix_fn: ident,
        $invoke_fn: ident,
        $invoke_signed_fn: ident,
        $ix_fn_with_program_id: ident,
        $invoke_fn_with_program_id: ident,
        $invoke_signed_fn_with_program_id: ident,

        $ix_with_program_id: ident,
        $accounts: ident,
        $keys: ident,
        $accounts_len: ident,
    ) => {
        use solana_program::{
            account_info::AccountInfo,
            entrypoint::ProgramResult,
            instruction::{AccountMeta, Instruction},
            program::{invoke, invoke_signed},
            pubkey::Pubkey,
        };
        use spl_token_interface::{$accounts, $accounts_len, $ix_with_program_id, $keys};

        pub fn $ix_fn(
            keys: $keys,
            signatories: impl Iterator<Item = Pubkey>,
        ) -> std::io::Result<Instruction> {
            $ix_fn_with_program_id(spl_token_interface::ID, keys, signatories)
        }

        pub fn $invoke_fn<'a, 'info>(
            accounts: $accounts<'a, 'info>,
            signatories: &'a [AccountInfo<'info>],
        ) -> ProgramResult {
            $invoke_fn_with_program_id(spl_token_interface::ID, accounts, signatories)
        }

        pub fn $invoke_signed_fn<'a, 'info>(
            accounts: $accounts<'a, 'info>,
            signatories: &'a [AccountInfo<'info>],
            seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            $invoke_signed_fn_with_program_id(spl_token_interface::ID, accounts, signatories, seeds)
        }

        pub fn $ix_fn_with_program_id(
            program_id: Pubkey,
            keys: $keys,
            signatories: impl Iterator<Item = Pubkey>,
        ) -> std::io::Result<Instruction> {
            let mut ix = $ix_with_program_id(program_id, keys)?;
            ix.accounts.last_mut().unwrap().is_signer = false;
            ix.accounts.extend(signatories.map(|pubkey| AccountMeta {
                pubkey,
                is_signer: true,
                is_writable: false,
            }));
            Ok(ix)
        }

        pub fn $invoke_fn_with_program_id<'a, 'info>(
            program_id: Pubkey,
            accounts: $accounts<'a, 'info>,
            signatories: &'a [AccountInfo<'info>],
        ) -> ProgramResult {
            let ix = $ix_fn_with_program_id(
                program_id,
                accounts.into(),
                signatories.iter().map(|a| *a.key),
            )?;
            let mut accounts = Vec::from(Into::<[AccountInfo; $accounts_len]>::into(accounts));
            accounts.extend(signatories.iter().cloned());
            invoke(&ix, &accounts)
        }

        pub fn $invoke_signed_fn_with_program_id<'a, 'info>(
            program_id: Pubkey,
            accounts: $accounts<'a, 'info>,
            signatories: &'a [AccountInfo<'info>],
            seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            let ix = $ix_fn_with_program_id(
                program_id,
                accounts.into(),
                signatories.iter().map(|a| *a.key),
            )?;
            let mut accounts = Vec::from(Into::<[AccountInfo; $accounts_len]>::into(accounts));
            accounts.extend(signatories.iter().cloned());
            invoke_signed(&ix, &accounts, seeds)
        }
    };
}

pub(crate) use multisig_impl;
pub(crate) use multisig_impl_no_ix_args;
