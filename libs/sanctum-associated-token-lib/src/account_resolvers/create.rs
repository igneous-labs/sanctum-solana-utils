use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{ReadonlyAccountOwner, ReadonlyAccountPubkey};
use spl_associated_token_account_interface::CreateKeys;

use crate::FindAtaAddressArgs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CreateFreeArgs<M: ReadonlyAccountPubkey + ReadonlyAccountOwner> {
    pub funding_account: Pubkey,
    pub wallet: Pubkey,
    pub mint: M,
}

impl<M: ReadonlyAccountPubkey + ReadonlyAccountOwner> CreateFreeArgs<M> {
    /// `.1` is bump seed of the ATA
    pub fn resolve(&self) -> (CreateKeys, u8) {
        let Self {
            funding_account,
            wallet,
            mint,
        } = self;
        let token_program = *mint.owner();
        let mint = *mint.pubkey();
        let (ata_to_create, bump) = FindAtaAddressArgs {
            wallet: *wallet,
            mint,
            token_program,
        }
        .find_ata_address();
        (
            CreateKeys {
                funding_account: *funding_account,
                associated_token_account: ata_to_create,
                wallet: *wallet,
                mint,
                system_program: system_program::ID,
                token_program,
            },
            bump,
        )
    }
}
