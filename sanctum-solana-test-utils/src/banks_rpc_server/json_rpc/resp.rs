use http_body_util::Full;
use hyper::body::Bytes;
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::response::{Response, RpcResponseContext};

use super::JsonRpc2Ident;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcResp<T> {
    pub jsonrpc: JsonRpc2Ident,
    pub id: u64,
    pub result: T,
}

impl<T> JsonRpcResp<T> {
    pub fn new(id: u64, result: T) -> Self {
        Self {
            jsonrpc: Default::default(),
            id,
            result,
        }
    }
}

impl<T> JsonRpcResp<Response<T>> {
    pub fn with_ctx(id: u64, value: T, slot: u64) -> Self {
        Self {
            jsonrpc: Default::default(),
            id,
            result: Response {
                context: RpcResponseContext::new(slot),
                value,
            },
        }
    }
}

impl<T: Serialize> From<JsonRpcResp<T>> for hyper::Response<Full<Bytes>> {
    fn from(value: JsonRpcResp<T>) -> Self {
        hyper::Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::to_vec(&value).unwrap().into())
            .unwrap()
    }
}
