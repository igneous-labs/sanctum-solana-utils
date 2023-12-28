mod account_resolvers;
mod mint_with_token_program;
mod readonly;

pub use account_resolvers::*;
pub use mint_with_token_program::*;
pub use readonly::*;

// These consts are `<Account/Mint as Packed>::LEN`,
// but just redefine them here so that we dont need to depend on spl-token

pub const SPL_TOKEN_ACCOUNT_PACKED_LEN: usize = 165;

pub const SPL_MINT_ACCOUNT_PACKED_LEN: usize = 82;
