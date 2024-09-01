use std::cell::RefMut;

use ark_bn254::Fr;
use sanctum_solana_kcsc::{fr_from_hash, fr_to_be, ToHash};
use sanctum_solana_kcsov::{
    eval_poly_pwrs_of_tau_g1, poly_from_roots, ByteBuf, KCSCCMut, KCSCCompress, KCSCDecompress,
    KCSCUOwned, KCSCC, KCSCU,
};
use sanctum_system_program_lib::{
    close_account, init_rent_exempt_account_invoke_signed, CloseAccountAccounts,
    InitRentExemptAccountArgs,
};
use solana_program::{
    account_info::AccountInfo, alt_bn128::compression::prelude::*, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
use system_program_interface::CreateAccountAccounts;

solana_program::entrypoint!(process);

sanctum_macros::declare_program_keys!(
    "BHeQ3kWSf4t4BLd3LbNXZ9efaUeziZnmw3eGsRsAf5is",
    [("kcsc", b"")]
);

const MAX_PROOFS_PER_IX: u8 = 3;

const MAX_PROOFS_PER_IX_P1: usize = MAX_PROOFS_PER_IX as usize + 1;

// You only need to store powers up to (1 + how many roots you wish to verify in one instruction) onchain
// tau = 2, only for testing. tau value must be discarded secret for actual use case.
const PWRS_OF_TAU_G1_COMPRESSED: [[u8; G1_COMPRESSED]; MAX_PROOFS_PER_IX_P1] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ],
    [
        3, 6, 68, 231, 46, 19, 26, 2, 155, 133, 4, 91, 104, 24, 21, 133, 217, 120, 22, 169, 22,
        135, 28, 168, 211, 194, 8, 193, 109, 135, 207, 211,
    ],
    [
        6, 167, 182, 74, 248, 244, 20, 188, 190, 239, 69, 91, 29, 165, 32, 140, 155, 89, 43, 131,
        238, 101, 153, 130, 76, 170, 109, 46, 233, 20, 26, 118,
    ],
    [
        136, 177, 213, 29, 35, 72, 12, 16, 244, 114, 245, 233, 59, 156, 254, 168, 130, 56, 193, 33,
        254, 21, 90, 247, 4, 57, 55, 136, 44, 48, 106, 99,
    ],
];

fn process(_program_id: &Pubkey, accounts: &[AccountInfo], ix_data: &[u8]) -> ProgramResult {
    let discm = ix_data
        .first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    let res = match discm {
        0 => process_init(accounts, ix_data),
        1 => process_consume(accounts, ix_data),
        _ => Err(ProgramError::InvalidInstructionData),
    };
    if let Err(e) = res.as_ref() {
        msg!("{}", e);
    }
    res
}

fn process_init(accounts: &[AccountInfo], ix_data: &[u8]) -> ProgramResult {
    let uncompressed: &[u8; G2] = ix_data[1..].try_into().unwrap();

    let payer = &accounts[0];
    let kcsc = &accounts[1];
    // let system_program = &accounts[2];

    init_rent_exempt_account_invoke_signed(
        CreateAccountAccounts {
            from: payer,
            to: kcsc,
        },
        InitRentExemptAccountArgs {
            space: G2_COMPRESSED,
            owner: ID,
        },
        &[&[&[KCSC_BUMP]]],
    )?;

    let kcsc_data = kcsc.try_borrow_mut_data()?;
    let mut kcsc_data = RefMut::filter_map(kcsc_data, |d| {
        <&mut [u8; G2_COMPRESSED]>::try_from(&mut d[..]).ok()
    })
    .map_err(|_e| ProgramError::AccountBorrowFailed)?;
    let into = KCSCCMut::new_unchecked(&mut kcsc_data);

    KCSCCompress::new(KCSCU::new_unchecked(uncompressed), into)
        .exec()
        .map_err(|_e| ProgramError::Custom(69))?;

    Ok(())
}

/// ix_data:
/// - [0] - discriminator
/// - [1..65] - compressed G2 proof point (pi)
/// - [65] - n_elems
/// - [66..] - n_elems of individual elements, packed. Each has length (ix_data.len() - 66) / n_elems
fn process_consume(accounts: &[AccountInfo], ix_data: &[u8]) -> ProgramResult {
    let kcsc = &accounts[0];
    if *kcsc.key != KCSC_ID {
        msg!("Wrong kcsc account");
        return Err(ProgramError::IncorrectProgramId);
    }

    let pi_compressed: &[u8; G2_COMPRESSED] = &ix_data[1..65].try_into().unwrap();
    let pi = alt_bn128_g2_decompress(pi_compressed).map_err(|e| {
        msg!("{}", e);
        ProgramError::Custom(70)
    })?;

    let n_elems = ix_data[65];
    if n_elems > MAX_PROOFS_PER_IX {
        msg!("MAX_PROOFS_PER_IX exceeded");
        return Err(ProgramError::InvalidInstructionData);
    }
    let n_elems_usize = usize::from(n_elems);
    let elems_data_subslice = &ix_data[66..];
    let elem_len = elems_data_subslice.len() / n_elems_usize;
    if elems_data_subslice.len() % n_elems_usize != 0 {
        msg!("ix_data.len() % n_elems not 0");
        return Err(ProgramError::InvalidInstructionData);
    }

    msg!("n_elems: {}. elem_len: {}", n_elems, elem_len);

    let mut roots: [Fr; MAX_PROOFS_PER_IX as usize] = Default::default();
    for (i, x) in elems_data_subslice.chunks_exact(elem_len).enumerate() {
        let hash = ByteBuf(x).to_hash();
        roots[i] = fr_from_hash(hash);
    }
    let roots = &roots[..n_elems_usize];
    let coeffs: [_; MAX_PROOFS_PER_IX_P1] = poly_from_roots(roots).map_err(|e| {
        msg!("{}", e);
        ProgramError::Custom(71)
    })?;
    let coeffs = &coeffs[..n_elems_usize + 1]; // dont forget the +1, coeffs = dp1 = roots + 1 !!!!

    let z_tau_g1 = eval_poly_pwrs_of_tau_g1(
        coeffs.iter().map(fr_to_be).zip(
            PWRS_OF_TAU_G1_COMPRESSED
                .iter()
                .map(|compressed| alt_bn128_g1_decompress(compressed).unwrap()),
        ),
    )
    .map_err(|e| {
        msg!("{}", e);
        ProgramError::Custom(72)
    })?;

    let mut kcscu = {
        let mut res = KCSCUOwned::new_unchecked([0u8; G2]);
        let d = kcsc.try_borrow_data()?;
        let kcscc = KCSCC::new_unchecked(d.as_ref().try_into().unwrap());
        KCSCDecompress::new(kcscc, res.borrowed_mut())
            .exec()
            .unwrap();
        res
    };

    kcscu
        .borrowed_mut()
        .consume_poly(&pi, &z_tau_g1)
        .map_err(|e| {
            msg!("{}", e);
            ProgramError::Custom(73)
        })?;

    if kcscu.borrowed().is_empty() {
        close_account(CloseAccountAccounts {
            refund_rent_to: kcsc,
            close: kcsc,
        })?;
    } else {
        let mut d = kcsc.try_borrow_mut_data()?;
        KCSCCompress::new(
            kcscu.borrowed(),
            KCSCCMut::new_unchecked(d.as_mut().try_into().unwrap()),
        )
        .exec()
        .unwrap();
    }

    Ok(())
}
