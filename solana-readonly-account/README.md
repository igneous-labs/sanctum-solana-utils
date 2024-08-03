# solana-readonly-account

Reimplementation of [ReadableAccount](https://docs.rs/solana-sdk/latest/solana_sdk/account/trait.ReadableAccount.html) to enable code reuse across off-chain clients ([solana-sdk](https://docs.rs/solana-sdk)) and on-chain programs ([solana-program](https://docs.rs/solana-program))

## Why was this crate created?

- You cannot use the original `ReadableAccount` trait from solana-sdk in on-chain programs because the solana-sdk feature flags don't work properly and it won't compile with `build-sbf`
- `Rc<RefCell<>>`s in [AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html) make it incompatible with `&[u8]` for `.data`

## Library

The 6 main account fields (key, lamports, data, owner, is_executable, rent_epoch) are split into a single getter trait each. This splitting allows for greater trait composability and flexibility.

For example, say you had a function that only requires the account's owner and this is a known static pubkey. Instead of having to fetch the full `Account` just to read its already-known owner field, or creating a dummy `Account`, you can simply define a newtype that only needs to implement `ReadonlyAccountOwner`, while still maintaining the ability to use this function with on-chain `AccountInfo`s.

Since [solana_sdk::Account](https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html) doesn't have its pubkey field, the following [`Keyed`](crate::keyed::Keyed) struct is defined in `feature = "keyed"` for off-chain use cases:

```rust ignore
pub struct Keyed<T> {
    pub pubkey: Pubkey,
    pub account: T,
}
```

A similar feature `keyed-bytes` exists that stores the pubkey as `[u8; 32]` instead for zero dependencies.

## Usage

Importing the respective traits from the crate now enables you to write generic functions that work both on-chain and off-chain

```rust
use solana_program::{
    program_error::ProgramError, program_pack::Pack,
};
use solana_readonly_account::ReadonlyAccountData;
use spl_token_2022::state::Account;

pub fn try_deserialize_token_account<A: ReadonlyAccountData>(
    acc: A,
) -> Result<Account, ProgramError> {
    Account::unpack(&acc.data())
}
```

By default, this crate has zero dependencies and only provides the trait definitions.

Feature `solana-pubkey` enables support for solana's `Pubkey` types on top of raw `[u8; 32]` types.

Feature `solana-program` impls the traits for `AccountInfo`

Feature `solana-sdk` impls the traits for `Account` and `AccountSharedData`. Do not enable this feature in an on-chain program crate, or `cargo-build-sbf` will fail.

## Testing

`cargo test --all-features`
