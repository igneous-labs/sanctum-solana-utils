#![cfg(feature = "solana-sdk")]

use solana_sdk::account::{Account, AccountSharedData, ReadableAccount};

use crate::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwnerBytes, ReadonlyAccountRentEpoch,
};

// impl for
// - Account
// - AccountSharedData
//
// Cant change to blanket impl for ReadableAccount, because:
// - impl conflicts with impl for AccountInfo, since upstream (solana) may impl ReadableAccount for AccountInfo in the future
// - impl conflicts with blanket impl for references

// Account

impl ReadonlyAccountLamports for Account {
    #[inline]
    fn lamports(&self) -> u64 {
        self.lamports
    }
}

impl ReadonlyAccountData for Account {
    type DataDeref<'d> = &'d [u8];

    #[inline]
    fn data(&self) -> Self::DataDeref<'_> {
        &self.data
    }
}

impl ReadonlyAccountOwnerBytes for Account {
    #[inline]
    fn owner_bytes(&self) -> [u8; 32] {
        self.owner.to_bytes()
    }
}

impl ReadonlyAccountIsExecutable for Account {
    #[inline]
    fn is_executable(&self) -> bool {
        self.executable
    }
}

impl ReadonlyAccountRentEpoch for Account {
    #[inline]
    fn rent_epoch(&self) -> u64 {
        self.rent_epoch
    }
}

// AccountSharedData

impl ReadonlyAccountLamports for AccountSharedData {
    #[inline]
    fn lamports(&self) -> u64 {
        <Self as ReadableAccount>::lamports(self)
    }
}

impl ReadonlyAccountData for AccountSharedData {
    type DataDeref<'d> = &'d [u8];

    #[inline]
    fn data(&self) -> Self::DataDeref<'_> {
        <Self as ReadableAccount>::data(self)
    }
}

impl ReadonlyAccountOwnerBytes for AccountSharedData {
    #[inline]
    fn owner_bytes(&self) -> [u8; 32] {
        <Self as ReadableAccount>::owner(self).to_bytes()
    }
}

impl ReadonlyAccountIsExecutable for AccountSharedData {
    #[inline]
    fn is_executable(&self) -> bool {
        <Self as ReadableAccount>::executable(self)
    }
}

impl ReadonlyAccountRentEpoch for AccountSharedData {
    #[inline]
    fn rent_epoch(&self) -> u64 {
        <Self as ReadableAccount>::rent_epoch(self)
    }
}

#[cfg(test)]
mod tests {
    use solana_program::program_pack::Pack;
    use solana_sdk::pubkey::Pubkey;
    use spl_token_2022::state::Account;

    use crate::test_utils::{gen_test_token_acc, try_deserialize_token_account};

    use super::*;

    #[test]
    fn test_token_acc_serde_roundtrip_account() {
        let acc = gen_test_token_acc();

        let mut data = vec![0u8; Account::LEN];
        Account::pack(acc, &mut data).unwrap();

        let account = solana_sdk::account::Account {
            lamports: 0,
            owner: Pubkey::default(),
            data,
            rent_epoch: 0,
            executable: false,
        };

        // blanket impl for ref
        let ref_deser = try_deserialize_token_account(&account).unwrap();
        assert_eq!(ref_deser, acc);

        // consume account
        let deser = try_deserialize_token_account(account).unwrap();
        assert_eq!(deser, acc);
    }

    #[test]
    fn test_token_acc_serde_roundtrip_account_shared_data() {
        let acc = gen_test_token_acc();

        let mut data = vec![0u8; Account::LEN];
        Account::pack(acc, &mut data).unwrap();

        let asd = AccountSharedData::from(solana_sdk::account::Account {
            lamports: 0,
            owner: Pubkey::default(),
            data,
            rent_epoch: 0,
            executable: false,
        });

        // blanket impl for ref
        let ref_deser = try_deserialize_token_account(&asd).unwrap();
        assert_eq!(ref_deser, acc);

        // consume account
        let deser = try_deserialize_token_account(asd).unwrap();
        assert_eq!(deser, acc);
    }
}
