# sanctum-solana-utils

Collection of utility libraries for onchain and offchain solana development.

## General Coding Guidlines

### Always pass `AccountInfo` by reference

`&AccountInfo`s are cheap, are `Copy`, and there's no reason to `clone()` them except on CPI.

### If a function takes 2 or more parameters of the same type, create an `Args` struct

So you dont have to think about getting the arg positions right.

Compare:

```rust ignore
transfer(account_a, account_b, 100);
```

to:

```rust ignore
transfer(
    TransferAccounts {
        from: account_a,
        to: account_b,
    },
    100,
)
```

In general for instructions you wanna split stuff up into `Accounts` and `Args` just like how solores does.

### Pass `Pubkey`s by value

TODO: profile performance impact of this.
