use ark_bn254::{Fr, G1Affine};
use ark_ff::{fields::Field, PrimeField};
use ark_serialize::CanonicalSerialize;
use sanctum_solana_kcsc::G1_GEN;
use sanctum_solana_test_utils::ExtendedProgramTest;
use solana_program_test::ProgramTest;
use solana_sdk::{instruction::Instruction, signer::Signer, transaction::Transaction};

#[tokio::test(flavor = "multi_thread")]
async fn sanity() {
    let mut pt = ProgramTest::default().add_upgradeable_program(
        kzg_hasher_test::ID,
        "kzg_hasher_test",
        None,
        0,
    );
    pt.prefer_bpf(true);
    let (mut bc, kp, hash) = pt.start().await;
    let mut tx = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            kzg_hasher_test::ID,
            &[169u8; 56],
            vec![],
        )],
        Some(&kp.pubkey()),
    );
    tx.sign(&[kp], hash);
    bc.process_transaction(tx).await.unwrap();
}

#[test]
fn sanity_repr_check() {
    let fr = Fr::ONE.double();
    let res = G1_GEN * fr;
    let res = G1Affine::from(res);
    eprintln!("{res}");
    let mut fr_bytes = vec![];
    fr.serialize_uncompressed(&mut fr_bytes).unwrap();
    eprintln!("{fr_bytes:?}");

    for u in res.x.into_bigint().0.iter().rev() {
        eprintln!("{:?}", u.to_be_bytes());
    }
    for u in res.y.into_bigint().0.iter().rev() {
        eprintln!("{:?}", u.to_be_bytes());
    }
}
