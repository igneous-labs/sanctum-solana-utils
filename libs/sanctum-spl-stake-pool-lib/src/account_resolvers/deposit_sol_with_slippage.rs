use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};
use solana_readonly_account::keyed::Keyed;
use spl_stake_pool_interface::{
    deposit_sol_with_slippage_ix_with_program_id, DepositSolWithSlippageIxArgs,
    DepositSolWithSlippageKeys, StakePool,
};

use crate::FindWithdrawAuthority;

#[derive(Clone, Copy, Debug)]
pub struct DepositSolWithSlippage<'a> {
    pub pool: Keyed<&'a StakePool>,
    pub deposit_from: Pubkey,
    pub mint_to: Pubkey,
    pub referral_fee_dest: Pubkey,
    pub lamports_in: u64,
    pub min_tokens_out: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DepositSolComputedKeys {
    pub withdraw_authority_pda: Pubkey,
}

impl<'a> DepositSolWithSlippage<'a> {
    pub fn compute_withdraw_auth(&self, program_id: &Pubkey) -> Pubkey {
        let (withdraw_authority_pda, _bump) = FindWithdrawAuthority {
            pool: self.pool.pubkey,
        }
        .run_for_prog(program_id);
        withdraw_authority_pda
    }

    pub fn compute_keys(&self, program_id: &Pubkey) -> DepositSolComputedKeys {
        DepositSolComputedKeys {
            withdraw_authority_pda: self.compute_withdraw_auth(program_id),
        }
    }

    pub fn full_ix(&self, program_id: &Pubkey) -> Result<Instruction, ProgramError> {
        let computed_keys = self.compute_keys(program_id);
        let mut ix = deposit_sol_with_slippage_ix_with_program_id(
            *program_id,
            self.resolve_with_computed_keys(computed_keys),
            DepositSolWithSlippageIxArgs {
                min_tokens_out: self.min_tokens_out,
                lamports_in: self.lamports_in,
            },
        )?;
        if let Some(sol_deposit_auth) = self.pool.account.sol_deposit_authority {
            ix.accounts.push(AccountMeta {
                pubkey: sol_deposit_auth,
                is_signer: true,
                is_writable: false,
            });
        }
        Ok(ix)
    }

    pub fn resolve_with_computed_keys(
        &self,
        DepositSolComputedKeys {
            withdraw_authority_pda,
        }: DepositSolComputedKeys,
    ) -> DepositSolWithSlippageKeys {
        let Self {
            pool:
                Keyed {
                    pubkey: stake_pool,
                    account:
                        StakePool {
                            reserve_stake,
                            pool_mint,
                            manager_fee_account,
                            token_program,
                            ..
                        },
                },
            mint_to,
            referral_fee_dest,
            deposit_from,
            ..
        } = self;
        DepositSolWithSlippageKeys {
            stake_pool: *stake_pool,
            withdraw_authority: withdraw_authority_pda,
            reserve_stake: *reserve_stake,
            mint_to: *mint_to,
            manager_fee_account: *manager_fee_account,
            pool_mint: *pool_mint,
            token_program: *token_program,
            deposit_from: *deposit_from,
            system_program: system_program::ID,
            referral_fee_dest: *referral_fee_dest,
        }
    }
}
