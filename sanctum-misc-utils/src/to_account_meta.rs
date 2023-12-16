use solana_program::{account_info::AccountInfo, instruction::AccountMeta};

pub trait ToAccountMeta {
    fn to_account_meta(&self) -> AccountMeta;
}

impl<T: ToAccountMeta> ToAccountMeta for &T {
    fn to_account_meta(&self) -> AccountMeta {
        (*self).to_account_meta()
    }
}

impl ToAccountMeta for AccountInfo<'_> {
    fn to_account_meta(&self) -> AccountMeta {
        AccountMeta {
            pubkey: *self.key,
            is_signer: self.is_signer,
            is_writable: self.is_writable,
        }
    }
}
