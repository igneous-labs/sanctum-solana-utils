//! A http server that runs solana RPC requests against a `BanksClient`
//! so that you can run CLI integration tests against `ProgramTest` instead of an actual cluster

use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Buf, Bytes, Incoming},
    rt::Executor,
    server::conn::http2,
    service::Service,
    Request, Response,
};
use hyper_util::rt::TokioIo;
use solana_account_decoder::{UiAccount, UiAccountEncoding};
use solana_program::{clock::Clock, hash::Hash, pubkey::Pubkey};
use solana_program_test::BanksClient;
use solana_rpc_client_api::config::RpcAccountInfoConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use std::{error::Error, future::Future, pin::Pin};
use tokio::{net::TcpListener, task::JoinHandle};

use crate::banks_rpc_server::json_rpc::{
    deser_get_multiple_accounts_params, JsonRpcReq, JsonRpcResp, RpcMethod,
};

use self::json_rpc::deser_get_latest_blockhash_params;

mod json_rpc;

#[derive(Clone)]
pub struct BanksRpcServer {
    // TODO: change this to BanksServer when solana makes it easier
    // to construct them from ProgramTest
    bc: BanksClient,
    rbh: Hash,
}

impl BanksRpcServer {
    /// Spawns the HTTP server on a random unused port and return the port
    pub async fn spawn_empty_ipv4(
        bc: BanksClient,
        rbh: Hash,
    ) -> (u16, JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>) {
        let s = Self { bc, rbh };
        for port in 1025..65535 {
            if let Ok(tcp_listener) = TcpListener::bind(("127.0.0.1", port)).await {
                return (port, s.spawn(tcp_listener));
            }
        }
        panic!("No available ports found");
    }

    /// Spawn the HTTP sever in the background
    pub fn spawn(
        self,
        tcp_listener: TcpListener,
    ) -> JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> {
        // just clone BanksClient on every req lul, it consists of some Arcs and atomics:
        // https://github.com/solana-labs/solana/blob/b78d41792aabe65a78a44d90174439b2f5579866/banks-server/src/banks_server.rs#L53
        tokio::task::spawn(async move {
            loop {
                let (tcp_stream, _socket_addr) = tcp_listener.accept().await?;
                let io = TokioIo::new(tcp_stream);
                let server = self.clone();
                tokio::task::spawn(async move {
                    if let Err(err) = http2::Builder::new(TokioExecutor)
                        .serve_connection(io, server)
                        .await
                    {
                        eprintln!("Error serving connection: {:?}", err);
                    }
                });
            }
        })
    }

    // TODO: using this to set context slot for responses is wrong, because the bank mightve advanced?
    pub async fn curr_slot(&mut self) -> u64 {
        let Clock { slot, .. } = self.bc.get_sysvar().await.unwrap();
        slot
    }

    // TODO: handle cfg, it just returns base64 encoded for now
    pub async fn get_multiple_accounts(
        &mut self,
        keys: Vec<Pubkey>,
        _cfg: Option<RpcAccountInfoConfig>,
    ) -> Result<Vec<Option<UiAccount>>, Box<dyn Error + Send + Sync>> {
        let mut res = Vec::with_capacity(keys.len());
        for key in keys {
            res.push(self.bc.get_account(key).await?.map(|account| {
                UiAccount::encode(&key, &account, UiAccountEncoding::Base64, None, None)
            }));
        }
        Ok(res)
    }

    // TODO: handle cfg, it just returns the blockhash it started with for now
    // TODO: convert to async if necessary
    pub fn get_latest_blockhash(&self, _cfg: Option<CommitmentConfig>) -> Hash {
        self.rbh
    }
}

impl Service<Request<Incoming>> for BanksRpcServer {
    type Response = Response<Full<Bytes>>;

    type Error = Box<dyn Error + Send + Sync>;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    // TODO: fix too many .clone()s
    // TODO: can create own future type instead of Pin<Box>
    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let mut this = self.clone();
        Box::pin(async move {
            let body = req.into_body().collect().await.ok().unwrap().to_bytes();
            let JsonRpcReq {
                jsonrpc: _,
                id,
                method,
                params,
            } = serde_json::from_reader(body.reader())?;
            Ok(match method {
                RpcMethod::GetMultipleAccounts => {
                    let (keys, cfg) = deser_get_multiple_accounts_params(params)?;
                    let value = this.get_multiple_accounts(keys, cfg).await?;
                    let resp = JsonRpcResp::new(id, value, this.curr_slot().await);
                    resp.into()
                }
                RpcMethod::GetLatestBlockhash => {
                    let cfg = deser_get_latest_blockhash_params(params)?;
                    let resp = JsonRpcResp::new(
                        id,
                        this.get_latest_blockhash(cfg),
                        this.curr_slot().await,
                    );
                    resp.into()
                }
            })
        })
    }
}

#[derive(Clone)]
struct TokioExecutor;

impl<F> Executor<F> for TokioExecutor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, future: F) {
        tokio::spawn(future);
    }
}
