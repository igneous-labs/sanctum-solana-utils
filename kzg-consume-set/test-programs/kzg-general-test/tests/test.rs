use ark_bn254::{Fr, G1Affine, G2Affine, G2Projective};
use ark_ff::{fields::Field, BigInteger256, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use kzg_general_test::KCSC_ID;
use sanctum_solana_kcsc::{fr_from_hash, ToHash, G1_GEN, G2_GEN};
use sanctum_solana_kcsov::{poly_from_roots, ByteBuf, G2_GEN_AFFINE_UNCOMPRESSED_BE};
use sanctum_solana_test_utils::{ExtendedBanksClient, ExtendedProgramTest};
use solana_program_test::ProgramTest;
use solana_sdk::{
    alt_bn128::compression::prelude::{G2, G2_COMPRESSED},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
    system_program,
    transaction::Transaction,
};

// tau = 2 for tests. Must be discarded secret for actual use case
fn tau() -> Fr {
    Fr::ONE.double()
}

fn program_test() -> ProgramTest {
    let mut pt = ProgramTest::default().add_upgradeable_program(
        kzg_general_test::ID,
        "kzg_general_test",
        None,
        0,
    );
    pt.prefer_bpf(true);
    pt
}

fn are_g2_pts_eq(ark: &G2Affine, mut be: [u8; G2]) -> bool {
    be[..G2 / 2].reverse();
    be[G2 / 2..].reverse();
    *ark == G2Affine::deserialize_uncompressed(be.as_slice()).unwrap()
}

fn init_ix(payer: Pubkey, commitment: &[u8; G2]) -> Instruction {
    let mut data = [0u8; G2 + 1];
    data[1..].copy_from_slice(commitment);
    Instruction::new_with_bytes(
        kzg_general_test::ID,
        &data,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: KCSC_ID,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: system_program::ID,
                is_signer: false,
                is_writable: false,
            },
        ],
    )
}

fn consume_ix<const N: usize>(
    proof: &[u8; G2_COMPRESSED],
    items: &[ByteBuf<[u8; N]>],
) -> Instruction {
    let mut data = vec![1u8];
    data.extend(proof);
    let n_elems = u8::try_from(items.len()).unwrap();
    data.push(n_elems);
    data.extend(items.iter().flat_map(|x| x.0));
    Instruction::new_with_bytes(
        kzg_general_test::ID,
        &data,
        vec![AccountMeta {
            pubkey: KCSC_ID,
            is_signer: false,
            is_writable: true,
        }],
    )
}

// TODO: adapt and move these util fns to kcsp
// - parameterize tau to powers of tau g2 instead
// - make fns fallible

fn g2_to_be_uncompressed(pt: &G2Projective) -> [u8; G2] {
    let mut buf = [0u8; G2];
    pt.serialize_uncompressed(buf.as_mut_slice()).unwrap();
    buf[..G2 / 2].reverse();
    buf[G2 / 2..].reverse();
    buf
}

fn g2_to_be_compressed(pt: &G2Projective) -> [u8; G2_COMPRESSED] {
    let mut buf = [0u8; G2_COMPRESSED];
    pt.serialize_compressed(buf.as_mut_slice()).unwrap();
    buf.reverse();
    buf
}

fn gen_commitment<const DP1: usize>(itr: impl IntoIterator<Item = impl ToHash>) -> G2Projective {
    // TODO: verify no Fr collisions
    let roots: Vec<_> = itr.into_iter().map(|x| fr_from_hash(x.to_hash())).collect();
    let coeffs = poly_from_roots::<DP1>(&roots).unwrap();
    coeffs.into_iter().enumerate().fold(
        G2Projective::from(G2Affine::identity()),
        |accum, (d, coeff)| {
            accum + G2_GEN * tau().pow(BigInteger256::from(u8::try_from(d).unwrap())) * coeff
        },
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn init_basic() {
    let pt = program_test();
    let (mut bc, kp, hash) = pt.start().await;
    let commitment = G2_GEN_AFFINE_UNCOMPRESSED_BE;
    let mut tx =
        Transaction::new_with_payer(&[init_ix(kp.pubkey(), &commitment)], Some(&kp.pubkey()));
    tx.sign(&[kp], hash);
    bc.process_transaction(tx).await.unwrap();

    let mut data = bc.get_account_data(KCSC_ID).await;
    data.as_mut_slice().reverse();
    let saved = G2Affine::deserialize_compressed(data.as_slice()).unwrap();

    assert!(are_g2_pts_eq(&saved, commitment));
}

#[tokio::test(flavor = "multi_thread")]
async fn single_elem() {
    let elems = [ByteBuf([0u8; 40])];

    let pt = program_test();
    let (mut bc, kp, hash) = pt.start().await;
    let commitment = gen_commitment::<2>(elems.iter());
    let mut tx = Transaction::new_with_payer(
        &[init_ix(kp.pubkey(), &g2_to_be_uncompressed(&commitment))],
        Some(&kp.pubkey()),
    );
    tx.sign(&[&kp], hash);
    bc.process_transaction(tx).await.unwrap();

    let mut data = bc.get_account_data(KCSC_ID).await;
    data.as_mut_slice().reverse();
    let saved = G2Affine::deserialize_compressed(data.as_slice()).unwrap();
    assert!(are_g2_pts_eq(&saved, g2_to_be_uncompressed(&commitment)));

    let ix = consume_ix(&g2_to_be_compressed(&G2Projective::from(G2_GEN)), &elems);
    assert_eq!(ix.data.len(), 106);
    let mut tx = Transaction::new_with_payer(&[ix], Some(&kp.pubkey()));
    tx.sign(&[&kp], hash);
    bc.process_transaction(tx).await.unwrap();

    let acc = bc.get_account(KCSC_ID).await.unwrap().unwrap();
    assert_eq!(acc.owner, system_program::ID);
    assert!(acc.data.is_empty());
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

#[test]
fn gen_pwrs_of_tau_g1() {
    const MIN_PWR: u8 = 0;
    const MAX_PWR: u8 = 3;

    (MIN_PWR..=MAX_PWR).for_each(|i| {
        let coeff = tau().pow(BigInteger256::from(i));
        let pt = G1_GEN * coeff;
        let mut buf = [0u8; 32];
        pt.serialize_compressed(buf.as_mut_slice()).unwrap();
        buf.reverse();
        eprintln!("{buf:?}");
    });
}
