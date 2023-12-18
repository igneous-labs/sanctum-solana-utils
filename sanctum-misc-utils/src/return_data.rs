use borsh::BorshDeserialize;
use solana_program::{program::get_return_data, pubkey::Pubkey};

/// Tries to read a borsh-serialized value from solana return data
///
/// Returns `None` if:
/// - no return data
/// - could not deserialize return data into the specified type
pub fn get_borsh_return_data<T: BorshDeserialize>() -> Option<(Pubkey, T)> {
    let (pk, data) = get_return_data()?;
    Some((pk, T::deserialize(&mut data.as_ref()).ok()?))
}
