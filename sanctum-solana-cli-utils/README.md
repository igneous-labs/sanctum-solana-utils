# sanctum-solana-cli-utils

Utils for building solana CLI applications.

## Testing

For all tests to run, make sure the solana CLI is installed locally since the tests test the default resolution of solana CLI config.

```sh
cargo test --all-features
```

## Features

- `clap` to enable `clap >= 3` dependency and enum value parser for `TxSendMode`
