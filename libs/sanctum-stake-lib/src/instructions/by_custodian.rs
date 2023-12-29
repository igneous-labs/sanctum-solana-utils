macro_rules! impl_by_custodian {
    (
        $ix_fn: ident,
        $invoke_fn: ident,
        $invoke_signed_fn: ident,

        $ix: ident,
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
        use stake_program_interface::{$accounts, $accounts_len, $ix, $ix_args, $keys};

        pub fn $ix_fn(keys: $keys, args: $ix_args, custodian: Pubkey) -> Instruction {
            let mut ix = $ix(keys, args);
            ix.accounts.push(AccountMeta {
                pubkey: custodian,
                is_signer: true,
                is_writable: false,
            });
            ix
        }

        pub fn $invoke_fn<'a, 'info>(
            accounts: $accounts<'a, 'info>,
            args: $ix_args,
            custodian: &'a AccountInfo<'info>,
        ) -> ProgramResult {
            let ix = $ix_fn(accounts.into(), args, *custodian.key);
            let mut accounts =
                Vec::from(Into::<[AccountInfo; $accounts_len]>::into(accounts).as_ref());
            accounts.push(custodian.clone());
            invoke(&ix, &accounts)
        }

        pub fn $invoke_signed_fn<'a, 'info>(
            accounts: $accounts<'a, 'info>,
            args: $ix_args,
            custodian: &'a AccountInfo<'info>,
            seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            let ix = $ix_fn(accounts.into(), args, *custodian.key);
            let mut accounts =
                Vec::from(Into::<[AccountInfo; $accounts_len]>::into(accounts).as_ref());
            accounts.push(custodian.clone());
            invoke_signed(&ix, &accounts, seeds)
        }
    };
}

pub(crate) use impl_by_custodian;
