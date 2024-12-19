/// All in terms of newly minted pool tokens
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DepositQuote {
    pub manager: u64,
    pub referrer: u64,
    pub user: u64,
}
