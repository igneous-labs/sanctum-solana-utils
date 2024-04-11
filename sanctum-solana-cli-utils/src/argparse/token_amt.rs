use std::{error::Error, fmt::Display, num::ParseIntError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenAmtParser {
    pub decimals: u8,
}

fn split_by_dot(s: &str) -> Result<(&str, &str), TokenAmtParseError> {
    let mut split = s.split('.');
    let integer_part = split.next().ok_or(TokenAmtParseError)?;
    let (has_decimal, decimal_part) = split.next().map_or_else(|| (false, ""), |s| (true, s));
    // need to check for has_decimal because some iterators
    // restart with a call to .next() after completion
    // and im too lazy to figure out if Split does that
    if has_decimal && split.next().is_some() {
        return Err(TokenAmtParseError);
    }
    Ok((integer_part, decimal_part))
}

fn parse_u64_empty_str_as_zero(s: &str) -> Result<u64, ParseIntError> {
    if s.is_empty() {
        Ok(0)
    } else {
        s.parse()
    }
}

impl TokenAmtParser {
    pub const fn new(decimals: u8) -> Self {
        Self { decimals }
    }

    pub const fn atomics_per_one(&self) -> u64 {
        10u64.pow(self.decimals as u32)
    }

    pub fn parse(&self, arg: &str) -> Result<TokenAmt, TokenAmtParseError> {
        let (int, dec) = split_by_dot(arg)?;

        let int: u64 = parse_u64_empty_str_as_zero(int)?;
        let int = int
            .checked_mul(self.atomics_per_one())
            .ok_or(TokenAmtParseError)?;

        if dec.len() > self.decimals.into() {
            return Err(TokenAmtParseError);
        }
        // subtraction overflow safety: decimals < len checked above
        // cast safety: both decimals and len are in u8 range
        let dec_mul = 10u64.pow(self.decimals as u32 - dec.len() as u32);
        let dec: u64 = parse_u64_empty_str_as_zero(dec)?
            .checked_mul(dec_mul)
            .ok_or(TokenAmtParseError)?;

        let amt = int.checked_add(dec).ok_or(TokenAmtParseError)?;
        Ok(TokenAmt {
            amt,
            decimals: self.decimals,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenAmtParseError;

impl Display for TokenAmtParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Expecting a decimal number with atomics within u64::MAX range and decimals not exceeding token's decimal precision. Examples for decimals=3: 234.123, 234",
        )
    }
}

impl Error for TokenAmtParseError {}

impl From<ParseIntError> for TokenAmtParseError {
    fn from(_v: ParseIntError) -> Self {
        Self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TokenAmt {
    pub amt: u64,
    pub decimals: u8,
}

impl TokenAmt {
    pub const fn atomics_per_one(&self) -> u64 {
        TokenAmtParser {
            decimals: self.decimals,
        }
        .atomics_per_one()
    }

    /// Returns how many ones are in this [`TokenAmt`]
    ///
    /// Example
    ///
    /// ```rust
    /// assert_eq!( sanctum_solana_cli_utils::TokenAmt { amt: 12_345, decimals: 3 }.integer_part(), 12);
    /// ```
    pub const fn integer_part(&self) -> u64 {
        // division safety: self.atomics_per_one() is never 0
        self.amt / self.atomics_per_one()
    }

    /// Returns the part after the decimal point in this [`TokenAmt`]
    ///
    /// Example
    ///
    /// ```rust
    /// assert_eq!( sanctum_solana_cli_utils::TokenAmt { amt: 12_345, decimals: 3 }.decimal_part(), 345);
    /// ```
    pub const fn decimal_part(&self) -> u64 {
        self.amt % self.atomics_per_one()
    }
}

impl Display for TokenAmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decimal_part = self.decimal_part();
        let decimals_width =
            std::iter::successors((decimal_part > 0).then_some(decimal_part), |&n| {
                (n >= 10).then_some(n / 10)
            })
            .count();
        if decimals_width == 0 {
            write!(f, "{}", self.integer_part())
        } else {
            // how many exact pwrs of 10 is the decimal part?
            // used to determine number of trailing zeroes to remove
            let (decimal_display, decimal_exact_pwr_ten): (u64, usize) =
                std::iter::successors(Some((decimal_part, 0)), |(n, accum)| {
                    (n % 10 == 0).then(|| (n / 10, accum + 1))
                })
                .last()
                .unwrap(); // unwrap safety, iterator always has at least 1 element since start with some
            write!(
                f,
                "{}.{:0display_decimals_width$}",
                self.integer_part(),
                decimal_display,
                display_decimals_width = usize::from(self.decimals) - decimal_exact_pwr_ten
            )
        }
    }
}

impl From<TokenAmt> for u64 {
    fn from(TokenAmt { amt, .. }: TokenAmt) -> Self {
        amt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let decimals = 3;
        let parser = TokenAmtParser::new(decimals);
        assert_eq!(
            parser.parse("69.234").unwrap(),
            TokenAmt {
                amt: 69_234,
                decimals
            }
        );
        assert_eq!(
            parser.parse("69.03").unwrap(),
            TokenAmt {
                amt: 69_030,
                decimals
            }
        );
        assert_eq!(
            parser.parse("69.2").unwrap(),
            TokenAmt {
                amt: 69_200,
                decimals
            }
        );
    }

    #[test]
    fn no_integer() {
        let decimals = 3;
        let parser = TokenAmtParser::new(decimals);
        assert_eq!(parser.parse(".2").unwrap(), TokenAmt { amt: 200, decimals });
        assert_eq!(parser.parse(".03").unwrap(), TokenAmt { amt: 30, decimals });
        assert_eq!(
            parser.parse("0.2").unwrap(),
            TokenAmt { amt: 200, decimals }
        );
        assert_eq!(
            parser.parse("0.03").unwrap(),
            TokenAmt { amt: 30, decimals }
        );
    }

    #[test]
    fn no_decimal() {
        let decimals = 3;
        let parser = TokenAmtParser::new(decimals);
        assert_eq!(
            parser.parse("1.").unwrap(),
            TokenAmt {
                amt: 1_000,
                decimals
            }
        );
        assert_eq!(
            parser.parse("3432").unwrap(),
            TokenAmt {
                amt: 3_432_000,
                decimals
            }
        );
    }

    #[test]
    fn err_cases() {
        let decimals = 3;
        let parser = TokenAmtParser::new(decimals);
        assert!(parser.parse("1..2").is_err());
        assert!(parser.parse("1.2.3").is_err());
        assert!(parser.parse("123,").is_err());
        assert!(parser.parse("1.2345").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(
            TokenAmt {
                amt: 0,
                decimals: 3
            }
            .to_string(),
            "0"
        );
        assert_eq!(
            TokenAmt {
                amt: 3_000,
                decimals: 3
            }
            .to_string(),
            "3"
        );
        assert_eq!(
            TokenAmt {
                amt: 69_100,
                decimals: 3
            }
            .to_string(),
            "69.1"
        );
        assert_eq!(
            TokenAmt {
                amt: 126_120,
                decimals: 3
            }
            .to_string(),
            "126.12"
        );
        assert_eq!(
            TokenAmt {
                amt: 100_020,
                decimals: 3
            }
            .to_string(),
            "100.02"
        );
        assert_eq!(
            TokenAmt {
                amt: 100_002,
                decimals: 3
            }
            .to_string(),
            "100.002"
        );
        assert_eq!(
            TokenAmt {
                amt: 546,
                decimals: 3
            }
            .to_string(),
            "0.546"
        );
    }
}
