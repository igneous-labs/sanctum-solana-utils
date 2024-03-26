use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountOwner, ReadonlyAccountPubkey};
use spl_stake_pool_interface::{InitializeKeys, INITIALIZE_IX_ACCOUNTS_LEN};

use crate::FindWithdrawAuthority;

pub const INITIALIZE_WITH_DEPOSIT_AUTH_IX_ACCOUNTS_LEN: usize = INITIALIZE_IX_ACCOUNTS_LEN + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InitializeFreeArgs<M> {
    pub pool_token_mint: M,
    pub stake_pool: Pubkey,
    pub manager: Pubkey,
    pub staker: Pubkey,
    pub validator_list: Pubkey,
    pub reserve_stake: Pubkey,
    pub manager_fee_account: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InitializeWithDepositAuthArgs {
    pub deposit_auth: Pubkey,
    pub program_id: Pubkey,
}

impl<M: ReadonlyAccountOwner + ReadonlyAccountPubkey> InitializeFreeArgs<M> {
    pub fn resolve_with_withdraw_auth(&self, withdraw_authority: Pubkey) -> InitializeKeys {
        InitializeKeys {
            stake_pool: self.stake_pool,
            manager: self.manager,
            staker: self.staker,
            withdraw_authority,
            validator_list: self.validator_list,
            reserve_stake: self.reserve_stake,
            pool_token_mint: *self.pool_token_mint.pubkey(),
            manager_fee_account: self.manager_fee_account,
            token_program: *self.pool_token_mint.owner(),
        }
    }

    pub fn resolve_for_prog(&self, program_id: &Pubkey) -> InitializeKeys {
        self.resolve_with_withdraw_auth(
            FindWithdrawAuthority {
                pool: self.stake_pool,
            }
            .run_for_prog(program_id)
            .0,
        )
    }

    pub fn resolve_with_deposit_auth(
        &self,
        InitializeWithDepositAuthArgs {
            deposit_auth,
            program_id,
        }: InitializeWithDepositAuthArgs,
    ) -> [AccountMeta; INITIALIZE_WITH_DEPOSIT_AUTH_IX_ACCOUNTS_LEN] {
        let [m0, m1, m2, m3, m4, m5, m6, m7, m8]: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] =
            self.resolve_for_prog(&program_id).into();
        [
            m0,
            m1,
            m2,
            m3,
            m4,
            m5,
            m6,
            m7,
            m8,
            AccountMeta {
                pubkey: deposit_auth,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
