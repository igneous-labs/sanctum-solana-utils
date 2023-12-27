# spl_token_interface

## Generate

Run in workspace root

```sh
solores \
    -o ./generated \
    --solana-program-vers "workspace=true" \
    --borsh-vers "workspace=true" \
    --thiserror-vers "workspace=true" \
    --num-derive-vers "workspace=true" \
    --num-traits-vers "workspace=true" \
    --serde-vers "workspace=true" \
    ./idl/spl_token.json
```

## Notes

- The original implementation on `solana-program-library` basically uses a handrolled borsh, making it compatible with a shank-style IDL
- `UiAmountToAmount` however is not compatible because they serialize the string without serializing the length as a u32 prefix. It is hence omitted.
- `COption`'s de/serialization in the instruction enums follows the borsh spec for `Option` (1-byte prefix for discriminant)
  - but for some reason `COption` is de/serialized with a 4-byte discriminant in accounts, therefore necessitating the `COptionPubkey` and `COptionU64` types
- `Transfer` is omitted due to it being deprecated in `token-2022`. Use `TransferChecked` instead.
- Where possible, `account` and `Account` are renamed to `token_account` and `TokenAccount` respectively to avoid collision with `solana_sdk::account::Account`
- Where possible, `owner` is renamed to `authority` to avoid collision with `accountInfo.owner`
- The various `*Checked` instructions have their instruction args type unified to the same struct, `CheckedOpArgs`
