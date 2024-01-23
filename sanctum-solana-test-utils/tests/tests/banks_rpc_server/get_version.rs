use solana_program_test::ProgramTest;

use crate::tests::banks_rpc_server::common::setup;

#[tokio::test(flavor = "multi_thread")]
async fn get_version_basic() {
    let (client, _payer, _rbh) = setup(ProgramTest::default()).await;
    client.get_version().unwrap();
}
