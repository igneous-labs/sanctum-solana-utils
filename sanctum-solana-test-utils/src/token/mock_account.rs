use solana_program::pubkey::Pubkey;

#[derive(Clone, Copy, Debug)]
pub struct MockTokenAccountArgs {
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub amount: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct MockMintArgs {
    pub mint_authority: Option<Pubkey>,
    pub freeze_authority: Option<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
}
