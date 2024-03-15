use solana_sdk::{
    address_lookup_table_account::AddressLookupTableAccount,
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signer::SignerError,
    signers::Signers,
    transaction::VersionedTransaction,
};

#[derive(Debug)]
pub struct ComputeBudgetParams {
    pub unit_limit: u32,
    pub unit_price_micro_lamports: u64,
}

pub fn create_versioned_transaction_with_compute_budget<T: Signers + ?Sized>(
    payer: &Pubkey,
    ixs: &[Instruction],
    address_lookup_table_accounts: &[AddressLookupTableAccount],
    recent_blockhash: Hash,
    signers: &T,
    ComputeBudgetParams {
        unit_limit,
        unit_price_micro_lamports,
    }: ComputeBudgetParams,
) -> Result<VersionedTransaction, SignerError> {
    let cb_ixs = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(unit_limit),
        ComputeBudgetInstruction::set_compute_unit_price(unit_price_micro_lamports),
    ];
    let ixs = [cb_ixs, ixs.to_vec()].concat();

    VersionedTransaction::try_new(
        VersionedMessage::V0(
            Message::try_compile(payer, &ixs, address_lookup_table_accounts, recent_blockhash)
                .unwrap(),
        ),
        signers,
    )
}

pub fn get_writable_account_keys(ixs: &[Instruction]) -> Vec<Pubkey> {
    let mut res = ixs
        .iter()
        .map(|ix| {
            ix.accounts
                .iter()
                .filter_map(|acc_meta| {
                    if acc_meta.is_writable {
                        Some(acc_meta.pubkey)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .concat();
    res.sort();
    res.dedup();
    res
}
