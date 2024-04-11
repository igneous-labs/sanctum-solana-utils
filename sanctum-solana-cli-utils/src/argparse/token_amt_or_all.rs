use std::fmt::Display;

use crate::{TokenAmt, TokenAmtParseError, TokenAmtParser};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenAmtOrAllParser(pub TokenAmtParser);

impl AsRef<TokenAmtParser> for TokenAmtOrAllParser {
    fn as_ref(&self) -> &TokenAmtParser {
        &self.0
    }
}

impl TokenAmtOrAllParser {
    pub const fn new(decimals: u8) -> Self {
        Self(TokenAmtParser::new(decimals))
    }

    pub const fn atomics_per_one(&self) -> u64 {
        self.0.atomics_per_one()
    }

    pub fn parse(&self, arg: &str) -> Result<TokenAmtOrAll, TokenAmtOrAllParseError> {
        Ok(if arg.to_lowercase() == "all" {
            TokenAmtOrAll::All {
                decimals: self.0.decimals,
            }
        } else {
            self.0.parse(arg)?.into()
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenAmtOrAllParseError;

impl Display for TokenAmtOrAllParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Expecting either case-insensitive 'all', or a decimal number with atomics within u64::MAX range and decimals not exceeding token's decimal precision. Examples for decimals=3: 234.123, 234",
        )
    }
}

impl From<TokenAmtParseError> for TokenAmtOrAllParseError {
    fn from(_v: TokenAmtParseError) -> Self {
        Self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenAmtOrAll {
    // its weird to have decimals repeated across the variants instead of extracting that out
    // to its own field but this is the way to get the struct size down to 2 words instead of 3
    Amt { decimals: u8, amt: u64 },
    All { decimals: u8 },
}

impl TokenAmtOrAll {
    pub const fn atomics_per_one(&self) -> u64 {
        let decimals = match self {
            Self::All { decimals } => decimals,
            Self::Amt { decimals, .. } => decimals,
        };
        TokenAmtOrAllParser::new(*decimals).atomics_per_one()
    }

    pub const fn is_all(&self) -> bool {
        match self {
            Self::All { .. } => true,
            Self::Amt { .. } => false,
        }
    }
}

impl Display for TokenAmtOrAll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All { .. } => f.write_str("all"),
            Self::Amt { decimals, amt } => write!(
                f,
                "{}",
                TokenAmt {
                    decimals: *decimals,
                    amt: *amt
                }
            ),
        }
    }
}

impl From<TokenAmt> for TokenAmtOrAll {
    fn from(TokenAmt { amt, decimals }: TokenAmt) -> Self {
        Self::Amt { amt, decimals }
    }
}
