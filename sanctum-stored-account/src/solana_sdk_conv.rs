use std::{error::Error, fmt::Display};

use solana_sdk::account::Account;

use crate::{ArcAccount, SmallAccount, StoredAccount, SMALL_ACCOUNT_DATA_MAX_LEN_USIZE};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataTooLong;

impl Display for DataTooLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account data too long")
    }
}

impl Error for DataTooLong {}

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
        let len = data.len();
        if len > SMALL_ACCOUNT_DATA_MAX_LEN_USIZE {
            return Err(DataTooLong);
        }
        let mut res = Self {
            data: Default::default(),
            len: len.try_into().unwrap(),
            lamports,
            rent_epoch,
            owner,
            executable,
        };
        res.data.copy_from_slice(&data);
        Ok(res)
    }
}

impl From<SmallAccount> for Account {
    fn from(value: SmallAccount) -> Self {
        let SmallAccount {
            data: _,
            len: _,
            lamports,
            rent_epoch,
            owner,
            executable,
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
