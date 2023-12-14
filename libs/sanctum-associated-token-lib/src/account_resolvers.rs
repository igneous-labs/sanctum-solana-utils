use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{CreateAtaKeys, FindAtaAddressArgs};

#[derive(Clone, Copy, Debug)]
pub struct CreateAtaFreeArgs<M: ReadonlyAccountPubkey + ReadonlyAccountOwner> {
    pub payer: Pubkey,
    pub wallet: Pubkey,
    pub mint: M,
}

impl<M: ReadonlyAccountPubkey + ReadonlyAccountOwner> CreateAtaFreeArgs<M> {
    pub fn resolve(&self) -> CreateAtaKeys {
        let Self {
            payer,
            wallet,
            mint,
        } = self;
        let token_program = *mint.owner();
        let mint = *mint.pubkey();
        let (ata_to_create, _bump) = FindAtaAddressArgs {
            wallet: *wallet,
            mint,
            token_program,
        }
        .find_ata_address();
        CreateAtaKeys {
            payer: *payer,
            ata_to_create,
            wallet: *wallet,
            mint,
            system_program: system_program::ID,
            token_program,
        }
    }
}
