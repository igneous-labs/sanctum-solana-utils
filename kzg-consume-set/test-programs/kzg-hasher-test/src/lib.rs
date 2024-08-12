use ark_bn254::Fr;
use ark_ff::fields::Field;
use sanctum_solana_kcsov::AltBn128G1ScalarMul;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, log::sol_log_compute_units, msg,
    pubkey::Pubkey,
};

solana_program::entrypoint!(process);

solana_program::declare_id!("BHeQ3kWSf4t4BLd3LbNXZ9efaUeziZnmw3eGsRsAf5is");

pub fn process(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    /*
    sol_log_compute_units(); // Each call to sol_log_compute_units is 100 CUs
    let hash = hashv(&[instruction_data]).to_bytes();
    sol_log_compute_units();
    let f = fr_from_hash(hash);
    sol_log_compute_units();
    let g = f * f; // fr mul takes ~100 CUs
    sol_log_compute_units();
    msg!("{}", g);
     */
    let fr = Fr::ONE.double();
    sol_log_compute_units();
    let res = AltBn128G1ScalarMul::new_zeros()
        .with_g1_gen()
        .with_fr(&fr)
        .exec()
        .unwrap();
    sol_log_compute_units();
    msg!("{:?}", &res[..32]);
    msg!("{:?}", &res[32..]);
    Ok(())
}
