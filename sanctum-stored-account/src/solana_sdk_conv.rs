use solana_sdk::account::Account;

use crate::{
    ArcAccount, DataTooLong, SmallAccount, SmallAccountTryNewParams, StoredAccount,
    SMALL_ACCOUNT_DATA_MAX_LEN_USIZE,
};

impl TryFrom<Account> for SmallAccount {
    type Error = DataTooLong;

    fn try_from(
        Account {
            lamports,
            data,
            owner,
            executable,
            rent_epoch,
        }: Account,
    ) -> Result<Self, Self::Error> {
        Self::try_new(SmallAccountTryNewParams {
            data: &data,
            lamports,
            rent_epoch,
            owner,
            executable,
        })
    }
}

impl From<SmallAccount> for Account {
    fn from(value: SmallAccount) -> Self {
        let SmallAccount {
            lamports,
            rent_epoch,
            owner,
            executable,
            ..
        } = value;
        Self {
            data: value.data_slice().into(),
            lamports,
            owner,
            executable,
            rent_epoch,
        }
    }
}

impl From<Account> for ArcAccount {
    fn from(
        Account {
            lamports,
            data,
            owner,
            executable,
            rent_epoch,
        }: Account,
    ) -> Self {
        Self {
            data: data.into(),
            lamports,
            rent_epoch,
            owner,
            executable,
        }
    }
}

impl From<ArcAccount> for Account {
    fn from(
        ArcAccount {
            data,
            lamports,
            rent_epoch,
            owner,
            executable,
        }: ArcAccount,
    ) -> Self {
        Self {
            data: (*data).into(),
            lamports,
            owner,
            executable,
            rent_epoch,
        }
    }
}

impl From<Account> for StoredAccount {
    fn from(value: Account) -> Self {
        if value.data.len() > SMALL_ACCOUNT_DATA_MAX_LEN_USIZE {
            Self::Arc(value.into())
        } else {
            Self::Small(value.try_into().unwrap())
        }
    }
}

impl From<StoredAccount> for Account {
    fn from(value: StoredAccount) -> Self {
        match value {
            StoredAccount::Arc(a) => a.into(),
            StoredAccount::Small(s) => s.into(),
        }
    }
}
