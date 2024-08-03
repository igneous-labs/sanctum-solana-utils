use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountOwnerBytes, ReadonlyAccountPubkeyBytes};
use spl_associated_token_account_interface::RecoverNestedKeys;

use crate::{CreateAtaAddressArgs, FindAtaAddressArgs};

pub struct RecoverNestedFreeArgs<M, N> {
    pub wallet: Pubkey,
    pub owner_token_account_mint: M,
    pub nested_mint: N,
}

impl<
        M: ReadonlyAccountPubkeyBytes + ReadonlyAccountOwnerBytes,
        N: ReadonlyAccountPubkeyBytes + ReadonlyAccountOwnerBytes,
    > RecoverNestedFreeArgs<M, N>
{
    /// Determins the spl-token program ID to use from the program owners of
    /// owner_token_account_mint and nested_mint
    /// Returns ProgramError::IllegalOwner if the 2 dont match
    pub fn det_token_program(&self) -> Result<Pubkey, ProgramError> {
        let owner_token_program =
            Pubkey::new_from_array(self.owner_token_account_mint.owner_bytes());
        let nested_token_program = Pubkey::new_from_array(self.nested_mint.owner_bytes());
        if owner_token_program != nested_token_program {
            return Err(ProgramError::IllegalOwner);
        }
        Ok(owner_token_program)
    }

    /// .1 is owner_token_account signer seeds args,
    /// required to pass into invoke_signed()
    pub fn resolve(&self) -> Result<(RecoverNestedKeys, CreateAtaAddressArgs), ProgramError> {
        let token_program = self.det_token_program()?;
        let keys = RecoverNestedFreeKeys {
            wallet: self.wallet,
            owner_token_account_mint: Pubkey::new_from_array(
                self.owner_token_account_mint.pubkey_bytes(),
            ),
            nested_mint: Pubkey::new_from_array(self.nested_mint.pubkey_bytes()),
            token_program,
        };
        Ok(keys.resolve())
    }
}

pub struct RecoverNestedFreeKeys {
    pub wallet: Pubkey,
    pub owner_token_account_mint: Pubkey,
    pub nested_mint: Pubkey,
    pub token_program: Pubkey,
}

impl RecoverNestedFreeKeys {
    /// .1 is owner_token_account signer seeds args,
    /// required to pass into invoke_signed()
    pub fn resolve(&self) -> (RecoverNestedKeys, CreateAtaAddressArgs) {
        let find_owner_token_account_args = FindAtaAddressArgs {
            wallet: self.wallet,
            mint: self.owner_token_account_mint,
            token_program: self.token_program,
        };
        let (owner_associated_token_account, bump) =
            find_owner_token_account_args.find_ata_address();
        let find_nested_token_account_args = FindAtaAddressArgs {
            wallet: owner_associated_token_account,
            mint: self.nested_mint,
            token_program: self.token_program,
        };
        let (nested, _) = find_nested_token_account_args.find_ata_address();
        let find_wallet_ata_args = FindAtaAddressArgs {
            wallet: self.wallet,
            mint: self.nested_mint,
            token_program: self.token_program,
        };
        let (wallet_associated_token_account, _) = find_wallet_ata_args.find_ata_address();
        (
            RecoverNestedKeys {
                wallet: self.wallet,
                owner_token_account_mint: self.owner_token_account_mint,
                nested_mint: self.nested_mint,
                token_program: self.token_program,
                nested,
                owner_associated_token_account,
                wallet_associated_token_account,
            },
            CreateAtaAddressArgs {
                find_ata_args: find_owner_token_account_args,
                bump,
            },
        )
    }
}
