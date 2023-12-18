#[cfg(feature = "onchain")]
mod onchain;
#[cfg(feature = "onchain")]
pub use onchain::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MathError;

#[cfg(feature = "std")]
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MathError")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MathError {}
