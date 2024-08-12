use solana_program::alt_bn128::compression::prelude::{G1, G2};

/// G1 generator (1, 2) in (x, y) form, big-endian.
///
/// This is the form expected by the sol_syscalls
pub const G1_GEN_AFFINE_UNCOMPRESSED_BE: [u8; G1] = {
    let mut res = [0u8; 64];
    res[31] = 0x1;
    res[63] = 0x2;
    res
};

/// G2 generator in (x, y) form, big-endian.
///
/// This is the form expected by the sol_syscalls
pub const G2_GEN_AFFINE_UNCOMPRESSED_BE: [u8; G2] = {
    // (x, y) little-endian output of G2_GEN.serialize_with_mode(.., ark_serialize::Compress::No)
    let mut res = [
        237, 246, 146, 217, 92, 189, 222, 70, 221, 218, 94, 247, 212, 34, 67, 103, 121, 68, 92, 94,
        102, 0, 106, 66, 118, 30, 31, 18, 239, 222, 0, 24, 194, 18, 243, 174, 183, 133, 228, 151,
        18, 231, 169, 53, 51, 73, 170, 241, 37, 93, 251, 49, 183, 191, 96, 114, 58, 72, 13, 146,
        147, 147, 142, 25, 170, 125, 250, 102, 1, 204, 230, 76, 123, 211, 67, 12, 105, 231, 209,
        227, 143, 64, 203, 141, 128, 113, 171, 74, 235, 109, 140, 219, 165, 94, 200, 18, 91, 151,
        34, 209, 220, 218, 172, 85, 243, 142, 179, 112, 51, 49, 75, 188, 149, 51, 12, 105, 173,
        153, 158, 236, 117, 240, 95, 88, 208, 137, 6, 9,
    ];
    let mut i = 0;
    while i < 2 {
        // iterate through half and swap [j] with [-j]
        let mut j = 0;
        while j < 32 {
            // swap not yet stable as const fn
            let head = i * 64 + j;
            let tail = i * 64 + 63 - j;
            let temp = res[head];
            res[head] = res[tail];
            res[tail] = temp;

            j += 1
        }
        i += 1;
    }
    res
};
