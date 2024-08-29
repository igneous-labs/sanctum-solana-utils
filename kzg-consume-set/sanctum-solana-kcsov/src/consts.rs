use solana_program::alt_bn128::compression::prelude::{G1, G2};

/// Size of base field in bytes
pub const FQ: usize = 32;

/// Size of scalar field in bytes
pub const FR: usize = 32;

// value obtained using
// little-endian output of ark_bn254::Fq::MODULUS.serialize_uncompressed.serialize_uncompressed(..)

/// Prime modulus of the curve's base field, big-endian
pub const Q_BE: [u8; FQ] = rev([
    71, 253, 124, 216, 22, 140, 32, 60, 141, 202, 113, 104, 145, 106, 129, 151, 93, 88, 129, 129,
    182, 69, 80, 184, 41, 160, 49, 225, 114, 78, 100, 48,
]);

/// `1`, big-endian
pub const G1_GEN_X_BE: [u8; FQ] = {
    let mut res = [0u8; FQ];
    res[FQ - 1] = 0x1;
    res
};

/// `2`, big-endian
pub const G1_GEN_Y_BE: [u8; FQ] = {
    let mut res = [0u8; FQ];
    res[FQ - 1] = 0x2;
    res
};

/// G1 generator (1, 2) in (x, y) form, big-endian.
///
/// This is the form expected by the sol_syscalls
pub const G1_GEN_AFFINE_UNCOMPRESSED_BE: [u8; G1] = coord_pair(G1_GEN_X_BE, G1_GEN_Y_BE);

/// Size of quadratic extension in bytes (2 * FQ)
pub const FQ2: usize = 64;

// values obtained using
// little-endian output of G2_GEN.serialize_with_mode(.., ark_serialize::Compress::No)

pub const G2_GEN_X_BE: [u8; FQ2] = rev([
    237, 246, 146, 217, 92, 189, 222, 70, 221, 218, 94, 247, 212, 34, 67, 103, 121, 68, 92, 94,
    102, 0, 106, 66, 118, 30, 31, 18, 239, 222, 0, 24, 194, 18, 243, 174, 183, 133, 228, 151, 18,
    231, 169, 53, 51, 73, 170, 241, 37, 93, 251, 49, 183, 191, 96, 114, 58, 72, 13, 146, 147, 147,
    142, 25,
]);

pub const G2_GEN_Y_BE: [u8; FQ2] = rev([
    170, 125, 250, 102, 1, 204, 230, 76, 123, 211, 67, 12, 105, 231, 209, 227, 143, 64, 203, 141,
    128, 113, 171, 74, 235, 109, 140, 219, 165, 94, 200, 18, 91, 151, 34, 209, 220, 218, 172, 85,
    243, 142, 179, 112, 51, 49, 75, 188, 149, 51, 12, 105, 173, 153, 158, 236, 117, 240, 95, 88,
    208, 137, 6, 9,
]);

/// G2 generator in (x, y) form, big-endian.
///
/// This is the form expected by the sol_syscalls
pub const G2_GEN_AFFINE_UNCOMPRESSED_BE: [u8; G2] = coord_pair(G2_GEN_X_BE, G2_GEN_Y_BE);

const fn coord_pair<const Q: usize, const Q2: usize>(x: [u8; Q], y: [u8; Q]) -> [u8; Q2] {
    let mut res = [0u8; Q2];

    let coords = [x, y];

    let mut i = 0;
    while i < 2 {
        let mut j = 0;
        let c = coords[i];
        while j < Q {
            res[i * Q + j] = c[j];
            j += 1;
        }
        i += 1;
    }

    res
}

const fn rev<const N: usize>(mut x: [u8; N]) -> [u8; N] {
    // iterate through half and swap [j] with [-j]
    let mut i = 0;
    while i < N / 2 {
        // swap not yet stable as const fn
        let head = i;
        let tail = N - 1 - i;
        let temp = x[head];
        x[head] = x[tail];
        x[tail] = temp;

        i += 1
    }
    x
}
