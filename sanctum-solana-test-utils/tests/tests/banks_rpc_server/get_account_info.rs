use sanctum_solana_test_utils::{ExtendedProgramTest, Keyed};
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::{account::Account, commitment_config::CommitmentConfig};

use crate::tests::banks_rpc_server::common::setup;

#[tokio::test(flavor = "multi_thread")]
async fn get_account_info_basic() {
    let pt = ProgramTest::default();

    let [p, o] = [(); 2].map(|_| Pubkey::new_unique());
    let d = vec![0, 1, 2, 3];

    let a = Account {
        lamports: 1_000_000,
        data: d,
        owner: o,
        executable: false,
        rent_epoch: 0,
    };
    let pt = pt.add_keyed_account(Keyed {
        pubkey: p,
        account: a.clone(),
    });

    let (client, _payer, _rbh) = setup(pt).await;

    assert_eq!(client.get_account(&p).unwrap(), a);
    assert_eq!(
        client
            .get_account_with_commitment(&Pubkey::new_unique(), CommitmentConfig::default())
            .unwrap()
            .value,
        None
    );
}
