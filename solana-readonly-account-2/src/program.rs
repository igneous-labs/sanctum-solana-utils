#![cfg(feature = "solana-program")]

use core::{cell::Ref, ops::Deref};

use solana_program::account_info::AccountInfo;

use crate::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwnerBytes, ReadonlyAccountPubkeyBytes, ReadonlyAccountRentEpoch,
};

impl ReadonlyAccountPubkeyBytes for AccountInfo<'_> {
    #[inline]
    fn pubkey_bytes(&self) -> [u8; 32] {
        self.key.to_bytes()
    }
}

impl ReadonlyAccountLamports for AccountInfo<'_> {
    #[inline]
    fn lamports(&self) -> u64 {
        AccountInfo::lamports(self)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AccountInfoDataRef<'a, 'r>(pub Ref<'r, &'a mut [u8]>);

impl<'a, 'r> Deref for AccountInfoDataRef<'a, 'r> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ReadonlyAccountData for AccountInfo<'a> {
    type DataDeref<'r> = AccountInfoDataRef<'a, 'r> where Self: 'r;

    /// panics if data is mutably borrowed
    ///
    /// Take note of lifetime of returned Ref;
    /// data cannot be borrow_mut() while it's not dropped
    #[inline]
    fn data(&self) -> Self::DataDeref<'_> {
        AccountInfoDataRef(self.data.borrow())
    }
}

impl ReadonlyAccountOwnerBytes for AccountInfo<'_> {
    #[inline]
    fn owner_bytes(&self) -> [u8; 32] {
        self.owner.to_bytes()
    }
}

impl ReadonlyAccountIsExecutable for AccountInfo<'_> {
    #[inline]
    fn is_executable(&self) -> bool {
        self.executable
    }
}

impl ReadonlyAccountRentEpoch for AccountInfo<'_> {
    #[inline]
    fn rent_epoch(&self) -> u64 {
        self.rent_epoch
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use solana_program::{program_pack::Pack, pubkey::Pubkey};
    use spl_token_2022::state::Account;

    use crate::test_utils::{gen_test_token_acc, try_deserialize_token_account};

    use super::*;

    #[test]
    fn test_token_acc_serde_roundtrip_account_info() {
        let acc = gen_test_token_acc();

        let mut data = [0u8; Account::LEN];
        Account::pack(acc, &mut data).unwrap();
        let mut lamports = 0;
        let key = Pubkey::default();
        let owner = Pubkey::default();

        let info = AccountInfo {
            key: &key,
            lamports: Rc::new(RefCell::new(&mut lamports)),
            owner: &owner,
            data: Rc::new(RefCell::new(&mut data)),
            rent_epoch: 0,
            is_signer: false,
            is_writable: false,
            executable: false,
        };

        // blanket impl for ref
        let ref_deser = try_deserialize_token_account(&info).unwrap();
        assert_eq!(ref_deser, acc);

        // consume info
        let deser = try_deserialize_token_account(info).unwrap();
        assert_eq!(deser, acc);
    }
}
