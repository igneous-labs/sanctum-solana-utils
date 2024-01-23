use solana_program_test::ProgramTest;

use crate::tests::banks_rpc_server::common::setup;

#[tokio::test(flavor = "multi_thread")]
async fn get_latest_blockhash_basic() {
    let (client, _payer, rbh) = setup(ProgramTest::default()).await;
    assert_eq!(client.get_latest_blockhash().unwrap(), rbh);
}
