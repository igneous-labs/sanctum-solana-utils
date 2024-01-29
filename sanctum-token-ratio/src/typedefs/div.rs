/// Indicates that any division operations in its main application should ceiling divide instead of floor
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct CeilDiv<T>(pub T);

impl<T> AsRef<T> for CeilDiv<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for CeilDiv<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

/// Indicates that any division operations in its main application should floor divide instead of ceiling
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct FloorDiv<T>(pub T);

impl<T> AsRef<T> for FloorDiv<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for FloorDiv<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
