# KZG ConsumeSet

Libraries for building a `KZGConsumeSet` data structure.

This is a HashSet that can only be initialized once with all elements in the set and the only operation/mutation thereafter is `consume()`, which deletes an element from the set.

Such a set can be stored offchain or in ledger data, while only the KZG commitment of the set needs to be stored in account data.

## Construction

### Overview

- Ellpitic curve used = bn254 aka altbn128.
- Hash function $h(x) = sha256(x) \ mod \ r$, where $sha256(x)$ is treated as an unsigned 256-bit integer and $r$ is the [prime order of bn254's scalar field](https://docs.rs/ark-bn254/latest/ark_bn254/). TODO: confirm that the modulus operation does not weaken sha256 hazardously.

### Prerequisites

- The set of $d$ elements to commit, $S = \{ e_1, e_2, ... , e_d \}$, must have elements of the same type that either implements `AsRef<[u8]>` or can compute a byte vec/array representation of itself
- The powers of tau ceremony must be of degree >= $d$

### Commit

- Each element in the set has its byte representation input into $h(x)$. Let $h_i$ be the output of $h(e_{i})$
- Commit the polynomial $p(x) = (x - h_1)(x - h_2)...(x - h_d) = \prod_{i=1}^{d} (x - h_i)$ and save the KZG commitment on-chain

### MultiVerify

To verify that the elements $S' = \{ f_1, f_2, ... , f_k \}$ are part of the committed set, the offchain prover produces a multi-KZG proof for $p(h(f_1)) = 0,\ p(h(f_2)) = 0,\ ...\ p(h(f_k)) = 0$. This is verified onchain against the stored KZG commitment.

### MultiConsume

To delete elements $S' = \{ f_1, f_2, ... , f_k \}$ that were previously in $S$ from $S$, the commitment should be updated to the multi-KZG proof verifying $S'$

### Empty

The committed set is empty if the commitment is equal to the generator of the commitment's elliptic curve.
