pub trait UncheckedCeilDiv {
    fn checked_ceil_div(&self, divisor: u128) -> Option<u128>;
}

impl UncheckedCeilDiv for u128 {
    fn checked_ceil_div(&self, divisor: u128) -> Option<u128> {
        self.checked_add(divisor)
            .and_then(|n| n.checked_sub(1))
            .and_then(|n| n.checked_div(divisor))
    }
}
