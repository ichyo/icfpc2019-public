use jsonrpc_client_http::{HttpHandle, HttpTransport};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockChainInfo {
    block: usize,
    block_subs: usize,
    block_ts: f64,
    total_subs: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MiningInfo {
    block: usize,
    excluded: Vec<usize>,
    puzzle: String,
    task: String,
}

jsonrpc_client!(pub struct LambdaClient {
    pub fn getblockchaininfo(&mut self) -> RpcRequest<BlockChainInfo>;
    pub fn getmininginfo(&mut self) -> RpcRequest<MiningInfo>;
    pub fn submit(&mut self, block: usize, task_sol_path: &str, pazzle_sol_path: &str) -> RpcRequest<Value>;
});

pub struct Client {
    api: LambdaClient<HttpHandle>,
}

impl Client {
    pub fn new() -> Client {
        let transport = HttpTransport::new().standalone().unwrap();
        let transport_handle = transport.handle("http://localhost:8332").unwrap();
        let client = LambdaClient::new(transport_handle);
        Client { api: client }
    }
}
