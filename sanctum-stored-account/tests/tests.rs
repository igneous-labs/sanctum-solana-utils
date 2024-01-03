use sanctum_stored_account::{ArcAccount, SmallAccount, StoredAccount};
use solana_sdk::account::Account;
use static_assertions::{const_assert, const_assert_eq};
use std::mem::{align_of, size_of};

const_assert_eq!(size_of::<ArcAccount>(), 72);
const_assert_eq!(size_of::<SmallAccount>(), size_of::<ArcAccount>());
const_assert_eq!(size_of::<StoredAccount>(), 80);
const_assert!(size_of::<StoredAccount>() <= size_of::<Account>());

const_assert_eq!(align_of::<ArcAccount>(), 8);
const_assert_eq!(align_of::<SmallAccount>(), 8);
const_assert_eq!(align_of::<StoredAccount>(), 8);
