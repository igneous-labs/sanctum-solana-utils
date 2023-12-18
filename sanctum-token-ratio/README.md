# sanctum-token-ratio

Utils for working with applying ratios to `u64` token amounts.

## Math

### "Inverting" a floor division

Many stake pools convert LST amount to SOL amount by taking

```
sol_amount = lst_amount * pool_sol // lst_supply
```

Let y = sol_amount, x = lst_amount, n = pool_sol, d = lst_supply.

Given y, n, d, find a suitable value of x

```
y = floor(nx/d)
y <= nx/d < y + 1
dy <= nx < d(y + 1)

LHS:
dy/n <= x

RHS:
x < d(y + 1) / n
x < dy/n + d/n

y(d/n) <= x < (y+1)(d/n)

x = ceil(dy/n) is a possible candidate, error at most 1 if RHS doesnt hold

When RHS doesn't hold:
Let r = dy%n, p be unknown

           | r/n |  d/n   | p |
floor(dy/n)     dy/n           ceil(dy/n)

p = 1 - r/n - d/n
  = (n-r-d)/n

Round to floor if r/n <= p , else ceil

If r/n <= p:
r <= n-r-d
2r <= n - d
```

Implemented in `U64RatioFloor::pseudo_reverse()`

### "Inverting" a floor fee charge

Many stake pools charge a percentage fee on stuff by taking

```
fee_amount = amount * fee_numerator // fee_denominator, output_amount = amount - fee_amount
```

Let y = output_amount, x = amount, n = fee_numerator, d = fee_denominator

Given y, n, d, find a suitable value of x

```
y = x - floor(nx/d)

floor(nx/d) = x - y
x-y <= nx/d < x-y+1

LHS:
x-y <= nx/d
dx-dy <= nx
dx - nx <= dy
x(d-n) <= dy
x <= dy/(d-n)

RHS:
nx/d < x-y+1
nx < dx-dy+d
dy-d < dx-nx
dy-d < x(d-n)
dy/(d-n) - d/(d-n) < x

(y-1)(d/(d-n)) < x <= y(d/(d-n))

d/(d-n) >= 1,
x = floor(dy/(d-n)) is always a possible candidate
```

Implemented in `U64FeeFloor::pseudo_reverse_from_amt_after_fee()`

### "Inverting" a floor fee charge from fees amount

```
fees = floor(nx/d)
```

Same as ["Inverting" a floor division](#inverting-a-floor-division) above.

### "Inverting a ceiling fee charge

Many stake pools charge a percentage fee on stuff by taking

```
amount_after_fee = amount * (fee_denominator - fee_numerator) // fee_denominator
fee = amount - amount_after_fee
```

Use ["Inverting" a floor division](#inverting-a-floor-division) with ratio = `(fee_denominator - fee_numerator) // fee_denominator` on `amount_after_fee`

### "Inverting" a ceiling fee charge from fees amount

Many stake pools charge a percentage fee on stuff by taking

```
amount_after_fee = amount * (fee_denominator - fee_numerator) // fee_denominator
fee = amount - amount_after_fee
```

Let f = fee, x = amount, n = fee_numerator, d = fee_denominator

```
f = x - floor(x(d-n)/d)
floor(x(d-n)/d) = x-f
floor(x - nx/d) = x-f
x-f <= x-nx/d < x-f+1

LHS:
x-f <= x-nx/d
nx/d <= f
x <= df/n

RHS:
x-nx/d < x-f+1
f-1 < nx/d
d(f-1)/n < x
df/n - d/n < x

(f-1)(d/n) < x <= f(d/n)

Since d >= n, d/n >= 1,
x = floor(df/n) is always a valid candidate
```
