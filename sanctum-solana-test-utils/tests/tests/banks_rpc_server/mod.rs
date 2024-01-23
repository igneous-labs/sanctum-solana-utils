// NB: if you use the blocking/sync solana RpcClient in the tests, you will need to annotate your tests with
// #[tokio::test(flavor = "multi_thread")] else it'll panic with
// `can call blocking only when running on the multi-threaded runtime`

mod common;
mod get_account_info;
mod get_latest_blockhash;
mod get_multiple_accounts;
mod get_version;
