use solana_sdk::signer::Signer;

/// Deduplicates a `Vec` of `Signer`s
pub fn dedup_signers(signers: &mut Vec<Box<dyn Signer>>) {
    signers.sort_by_key(|l| l.pubkey());
    signers.dedup_by(|a, b| a.pubkey() == b.pubkey());
}
