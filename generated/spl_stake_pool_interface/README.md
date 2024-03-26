# spl_stake_pool_interface

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
    ./idl/spl_stake_pool.json
```
