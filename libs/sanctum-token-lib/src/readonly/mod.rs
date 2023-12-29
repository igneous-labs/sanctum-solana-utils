mod mint;
mod token_account;

pub use mint::*;
pub use token_account::*;

pub const COPTION_NONE_DISCM: [u8; 4] = [0; 4];

pub const COPTION_SOME_DISCM: [u8; 4] = [1, 0, 0, 0];

/// # Returns:
/// - Some(slice[4..]) if COption::Some
/// - None if COption::None
///
/// # Panics
/// - If discriminant slice[..4] does not match [`COPTION_NONE_DISCM`] or [`COPTION_SOME_DISCM`]
pub fn unpack_coption_slice(slice: &[u8]) -> Option<&[u8]> {
    let discm: &[u8; 4] = slice[..4].try_into().unwrap();
    match *discm {
        COPTION_NONE_DISCM => None,
        COPTION_SOME_DISCM => Some(&slice[4..]),
        _ => panic!("Invalid COption discm {discm:?}"),
    }
}

pub fn is_coption_discm_valid(discm: &[u8; 4]) -> bool {
    matches!(*discm, COPTION_NONE_DISCM | COPTION_SOME_DISCM)
}

#[cfg(test)]
mod test_utils {
    use solana_sdk::account::Account;

    pub fn to_account(bytes: &[u8]) -> Account {
        Account {
            lamports: 0,
            data: bytes.to_vec(),
            owner: spl_token_2022::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}
