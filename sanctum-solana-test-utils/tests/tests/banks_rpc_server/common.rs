use sanctum_solana_test_utils::banks_rpc_server::BanksRpcServer;
use solana_client::rpc_client::RpcClient;
use solana_program::hash::Hash;
use solana_program_test::ProgramTest;
use solana_sdk::signature::Keypair;

pub async fn setup(pt: ProgramTest) -> (RpcClient, Keypair, Hash) {
    let (bc, payer, rbh) = pt.start().await;
    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc).await;
    let client = RpcClient::new(format!("http://127.0.0.1:{port}"));
    (client, payer, rbh)
}
