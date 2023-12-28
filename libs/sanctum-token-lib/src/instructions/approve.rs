super::multisig_impl::multisig_impl!(
    approve_multisig_ix_with_program_id,
    approve_multisig_invoke_with_program_id,
    approve_multisig_invoke_signed_with_program_id,
    approve_ix_with_program_id,
    ApproveAccounts,
    ApproveKeys,
    ApproveIxArgs,
    APPROVE_IX_ACCOUNTS_LEN,
);

#[cfg(test)]
mod tests {
    use spl_token_2022::instruction::approve;
    use spl_token_interface::{ApproveIxArgs, ApproveKeys};

    use super::*;

    #[test]
    fn approve_multisig_serde() {
        let amount = 69;
        let token_account = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let signatories = [
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        ];
        // spl_token_2022::approve expects &[&Pubkey]
        let signatories_ref: [&Pubkey; 3] = [&signatories[0], &signatories[1], &signatories[2]];

        let actual = approve_multisig_ix_with_program_id(
            spl_token_2022::ID,
            ApproveKeys {
                token_account,
                delegate,
                authority,
            },
            ApproveIxArgs { amount },
            signatories.iter().copied(),
        )
        .unwrap();
        let expected = approve(
            &spl_token_2022::ID,
            &token_account,
            &delegate,
            &authority,
            &signatories_ref,
            amount,
        )
        .unwrap();
        assert_eq!(actual, expected);
    }
}
