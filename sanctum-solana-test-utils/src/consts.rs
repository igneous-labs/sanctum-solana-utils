pub const ZERO_SIZE_RENT_EXEMPT_LAMPORTS: u64 = 890880;

pub const RENT_EXEMPT_LAMPORT_PER_BYTE: u64 = 6960;

pub const fn est_rent_exempt_lamports(account_data_len: usize) -> u64 {
    ZERO_SIZE_RENT_EXEMPT_LAMPORTS + account_data_len as u64 * RENT_EXEMPT_LAMPORT_PER_BYTE
}
