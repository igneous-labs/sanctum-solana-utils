mod token_account;

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
