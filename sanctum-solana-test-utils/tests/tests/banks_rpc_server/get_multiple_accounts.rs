use sanctum_solana_test_utils::{ExtendedProgramTest, Keyed};
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::account::Account;

use crate::tests::banks_rpc_server::common::setup;

#[tokio::test(flavor = "multi_thread")]
async fn get_multiple_accounts_basic() {
    let pt = ProgramTest::default();

    let [p1, p2, o1, o2] = [(); 4].map(|_| Pubkey::new_unique());
    let d1 = vec![0, 1, 2, 3];
    let d2 = vec![0; 256];

    let a1 = Account {
        lamports: 1_000_000,
        data: d1,
        owner: o1,
        executable: false,
        rent_epoch: 0,
    };
    let a2 = Account {
        lamports: 1_000_000,
        data: d2,
        owner: o2,
        executable: true,
        rent_epoch: 0,
    };
    let pt = pt
        .add_keyed_account(Keyed {
            pubkey: p1,
            account: a1.clone(),
        })
        .add_keyed_account(Keyed {
            pubkey: p2,
            account: a2.clone(),
        });

    let (client, _payer, _rbh) = setup(pt).await;

    assert_eq!(
        client.get_multiple_accounts(&[p1, p2]).unwrap(),
        Vec::from([a1, a2].map(Some))
    );
}
