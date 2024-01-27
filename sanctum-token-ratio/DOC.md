# Doc

Random notes

## Equivalence In Application of Fees

In general there are 2 ways of applying fee ratios:

1. take `fees_charged = amount * fee_num / fee_denom` and `amount_after_fee = amount - fee_charged`
2. take `amount_after_fee = amount * (fee_denom - fee_num) / fee_num` and `fee_charged = amount - amount_after_fee`

Where the division in both cases can either be floor or ceil.

They are actually equivalent, the ceiling div of one is the floor div of the other.

Let `n` be a `fee_num` and `d` be a `fee_denom` e.g. n = 1, d = 10 means a 10% fee.

Let `y` be amount after fees, `x` be amount before fees, `f` be fee amount

### Show ceil of method 1 is equivalent to floor of method 2

floor method 2:

```md
y = floor(x(d - n)/d)
```

ceil method 1:

```md
f = ceil(xn/d)
y = x - ceil(xn/d)
ceil(xn/d) = x - y
xn/d <= x - y < xn/d + 1

LHS:
y <= x(1 - n/d)

RHS:
x(1 - n/d) - 1 < y

x(1 - n/d) - 1 < y <= x(1 - n/d)
x(d - n)/d - 1 < y <= x(d - n)/d
y = floor(x(d - n)/d)
```

### Show floor of method 1 is equivalent to ceil of method 2

ceil method 2:

```md
y = ceil(x(d - n)/d)
```

floor method 1:

```md
f = floor(xn/d)
y = x - floor(xn/d)
floor(xn/d) = x - y
x - y <= xn/d < x - y + 1

LHS:
x(1 - n/d) <= y

RHS:
y < x(1 - n/d) + 1

x(1 - n/d) <= y < x(1 - n/d) + 1
x(d - n)/d <= y < x(d - n)/d + 1
y = ceil(x(d - n)/d)
```

Old impl of `U64FeeRem` (method 2) + proptest showing this equivalence

```rust
use crate::{
    AmtsAfterFee, AmtsAfterFeeBuilder, CeilDiv, FeeRatio, FeeRatioBounds, FeeRatioRem, FloorDiv,
    MathError, ReversibleFee, ReversibleRatio, U64FeeRem, U64ValueRange,
};

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> ReversibleFee for FloorDiv<U64FeeRem<N, D>> {
    fn apply(&self, amt_before_fee: u64) -> Result<AmtsAfterFee, MathError> {
        let amt_after_fee = FloorDiv(self.0.one_minus_fee_ratio()?).apply(amt_before_fee)?;
        AmtsAfterFeeBuilder::new_amt_bef_fee(amt_before_fee).with_amt_aft_fee(amt_after_fee)
    }

    fn reverse_from_amt_after_fee(&self, amt_after_fee: u64) -> Result<U64ValueRange, MathError> {
        let Self(fee) = self;
        if fee.is_zero() {
            Ok(U64ValueRange::single(amt_after_fee))
        } else {
            FloorDiv(fee.one_minus_fee_ratio()?).reverse(amt_after_fee)
        }
    }

    fn reverse_from_fee_charged(&self, fee_charged: u64) -> Result<U64ValueRange, MathError> {
        let Self(fee) = self;
        if fee.is_max() {
            Ok(U64ValueRange::single(fee_charged))
        } else {
            CeilDiv(fee.to_u64_ratio()).reverse(fee_charged)
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use crate::U64FeeDirect;

    use super::*;

    prop_compose! {
        pub fn valid_fees()
            (fee_denom in any::<u64>())
            (fee_num in 0..=fee_denom, fee_denom in Just(fee_denom)) -> (U64FeeRem<u64, u64>, U64FeeDirect<u64, u64>) {
                (
                    U64FeeRem::from_fee_num_and_denom(fee_num, fee_denom).unwrap(),
                    U64FeeDirect::from_fee_num_and_denom(fee_num, fee_denom).unwrap()
                )
            }
    }

    proptest! {
        #[test]
        fn equivalency(amt_before_fee: u64, (fr, fd) in valid_fees()) {
            let fr = FloorDiv(fr);
            let fd = CeilDiv(fd);
            let fr_r = fr.apply(amt_before_fee);
            let fd_r = fd.apply(amt_before_fee);
            prop_assert_eq!(fr_r, fd_r);

            if let Ok(a) = fr_r {
                prop_assert_eq!(fr.reverse_from_amt_after_fee(a.amt_after_fee()), fd.reverse_from_amt_after_fee(a.amt_after_fee()));
                prop_assert_eq!(fr.reverse_from_fee_charged(a.fee_charged()), fd.reverse_from_fee_charged(a.fee_charged()));
            }
        }
    }
}
```
