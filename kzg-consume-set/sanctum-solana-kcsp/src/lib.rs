#[cfg(test)]
mod tests {
    use ark_bn254::{
        g1::{G1_GENERATOR_X, G1_GENERATOR_Y},
        g2::{G2_GENERATOR_X, G2_GENERATOR_Y},
        Config, Fq12, Fr, G1Affine, G2Affine,
    };
    use ark_ec::{bn::Bn, pairing::Pairing, CurveGroup};
    use ark_ff::{Field, One, Zero};
    use proptest::{prop_assert_eq, prop_assert_ne, proptest};
    use sanctum_solana_kcsc::fr_from_hash;

    const G1_GEN: G1Affine = G1Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);
    const G2_GEN: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    #[test]
    fn print_g2_gen() {
        use ark_serialize::CanonicalSerialize;
        let mut buf = [0u8; 128];
        G2_GEN
            .serialize_with_mode(&mut buf[..], ark_serialize::Compress::No)
            .unwrap();
        eprintln!("{buf:?}");
    }

    proptest! {
        #[test]
        fn quadratic_sanity(tau: [u8; 32], r1: [u8; 32], r2: [u8; 32], fake: [u8; 32], fake_proof_scalar: [u8; 32]) {
            let [tau, r1, r2, fake, fake_proof_scalar] = [tau, r1, r2, fake, fake_proof_scalar].map(fr_from_hash);
            let is_fake_diff = fake != r1 && fake != r2;
            if tau.is_zero() || tau.is_one() {
                return Ok(())
            }

            let tau2 = tau.square();

            let pwrs_of_tau = [G2_GEN * Fr::ONE, G2_GEN * tau, G2_GEN * tau2];

            let commit_coeffs = [r1 * r2, -r1 - r2, Fr::ONE];
            let commit_g2_pt = commit_coeffs.into_iter()
                .zip(pwrs_of_tau.into_iter())
                .map(|(c, pwr_of_tau)| pwr_of_tau * c)
                .reduce(|a, b| a + b).unwrap();

            let expected_pairing = <Bn<Config> as Pairing>::pairing(G1_GEN, commit_g2_pt);

            // r1 proof
            let p1 = G2_GEN * tau - G2_GEN * r2;
            let p1_arg = G1_GEN * tau - G1_GEN * r1;
            let p1_pairing = <Bn<Config> as Pairing>::pairing(p1_arg, p1);
            prop_assert_eq!(expected_pairing, p1_pairing);
            prop_assert_eq!(Fq12::ONE, <Bn<Config> as Pairing>::multi_pairing([-G1_GEN, p1_arg.into_affine()], [commit_g2_pt, p1]).0);

            // r2 proof
            let p2 = G2_GEN * tau - G2_GEN * r1;
            let p2_arg = G1_GEN * tau - G1_GEN * r2;
            let p2_pairing = <Bn<Config> as Pairing>::pairing(p2_arg, p2);
            prop_assert_eq!(expected_pairing, p2_pairing);
            prop_assert_eq!(Fq12::ONE, <Bn<Config> as Pairing>::multi_pairing([-G1_GEN, p2_arg.into_affine()], [commit_g2_pt, p2]).0);

            // r1&r2 simultaneous proof
            let pboth = G2_GEN;
            let pboth_arg = commit_coeffs.into_iter()
                .zip([G1_GEN * Fr::ONE, G1_GEN * tau, G1_GEN * tau2].into_iter())
                .map(|(c, pwr_of_tau)| pwr_of_tau * c)
                .reduce(|a, b| a + b).unwrap();
            let pboth_pairing = <Bn<Config> as Pairing>::pairing(pboth_arg, pboth);
            prop_assert_eq!(expected_pairing, pboth_pairing);
            prop_assert_eq!(Fq12::ONE, <Bn<Config> as Pairing>::multi_pairing([-G1_GEN, pboth_arg.into_affine()], [commit_g2_pt, pboth.into()]).0);

            if is_fake_diff {
                let pfake = G2_GEN * fake_proof_scalar;
                let pfake_arg = G1_GEN * tau - G1_GEN * fake;
                let pfake_pairing = <Bn<Config> as Pairing>::pairing(pfake_arg, pfake);
                prop_assert_ne!(expected_pairing, pfake_pairing);
                prop_assert_ne!(Fq12::ONE, <Bn<Config> as Pairing>::multi_pairing([-G1_GEN, pfake_arg.into_affine()], [commit_g2_pt, pfake]).0);
            }

            // consume r2 then prove r1
            let p1_only_commit = p2;
            let p1_only = G2_GEN;
            let p1_only_pairing = <Bn<Config> as Pairing>::pairing(p1_arg, p1_only);
            prop_assert_eq!(<Bn<Config> as Pairing>::pairing(G1_GEN, p1_only_commit), p1_only_pairing);
            prop_assert_eq!(Fq12::ONE, <Bn<Config> as Pairing>::multi_pairing([-G1_GEN, p1_arg.into()], [p1_only_commit, p1_only.into()]).0);

            // consume r1 then prove r2
            let p2_only_commit = p1;
            let p2_only = G2_GEN;
            let p2_only_pairing = <Bn<Config> as Pairing>::pairing(p2_arg, p2_only);
            prop_assert_eq!(<Bn<Config> as Pairing>::pairing(G1_GEN, p2_only_commit), p2_only_pairing);
            prop_assert_eq!(Fq12::ONE, <Bn<Config> as Pairing>::multi_pairing([-G1_GEN, p2_arg.into()], [p2_only_commit, p2_only.into()]).0);
        }
    }
}
