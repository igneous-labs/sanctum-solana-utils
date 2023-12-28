# system_program_interface

## Generate

Run in workspace root

```sh
solores \
    -o ./generated \
    --solana-program-vers "workspace=true" \
    --thiserror-vers "workspace=true" \
    --num-derive-vers "workspace=true" \
    --num-traits-vers "workspace=true" \
    --serde-vers "workspace=true" \
    ./idl/system.json
```
