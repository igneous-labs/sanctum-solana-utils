#![cfg_attr(not(test), no_std)]

use ark_bn254::{
    g1::{G1_GENERATOR_X, G1_GENERATOR_Y},
    g2::{G2_GENERATOR_X, G2_GENERATOR_Y},
    G1Affine, G2Affine,
};

mod conv;
mod hash;

pub use conv::*;
pub use hash::*;

/// Can be obtained with `<G1Affine as ark_ec::AffineRepr>::generator()` but re-exported for use in const contexts
pub const G1_GEN: G1Affine = G1Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

/// Can be obtained with `<G2Affine as ark_ec::AffineRepr>::generator()` but re-exported for use in const contexts
pub const G2_GEN: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);
