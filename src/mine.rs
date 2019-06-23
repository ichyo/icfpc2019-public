use jsonrpc_client_http::{HttpHandle, HttpTransport};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{thread, time};

use crate::models::*;
use crate::parse::{read_puzzle, read_task};
use crate::puzzle::solve_pazzle;
use crate::solve::solve_small;

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
    //excluded: Vec<usize>,
    puzzle: String,
    task: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockInfo {
    block: usize,
    puzzle: String,
    task: String,
}

jsonrpc_client!(pub struct LambdaClient {
    pub fn getblockchaininfo(&mut self) -> RpcRequest<BlockChainInfo>;
    pub fn getmininginfo(&mut self) -> RpcRequest<MiningInfo>;
    pub fn getblockinfo(&mut self, block: usize) -> RpcRequest<BlockInfo>;
    pub fn submit(&mut self, block: usize, task_sol_path: &str, pazzle_sol_path: &str) -> RpcRequest<Value>;
});

pub struct Client {
    api: LambdaClient<HttpHandle>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Client {
        let transport = HttpTransport::new().standalone().unwrap();
        let transport_handle = transport.handle("http://localhost:8332").unwrap();
        let client = LambdaClient::new(transport_handle);
        Client { api: client }
    }

    pub fn solve(&self, block: usize, puzzle: &str, task: &str) {
        let mut rand = thread_rng();
        let puzzle = read_puzzle(puzzle);
        eprintln!("{:?}", puzzle);
        let task = read_task(task);
        //let commands = solve_small(task);

        let puzzle_answer = solve_pazzle(puzzle);
        println!("{:?}", puzzle_answer);
    }

    pub fn execute(&mut self) {
        loop {
            match self.api.getmininginfo().call() {
                Ok(x) => {
                    for b in 0..x.block {
                        let b = self.api.getblockinfo(b).call().unwrap();
                        let puzzle = read_puzzle(&b.puzzle);
                        let puzzle_answer = solve_pazzle(puzzle);
                        println!("{:?}", puzzle_answer);
                    }
                    self.solve(x.block, &x.puzzle, &x.task);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }

            let one_sec = time::Duration::from_secs(60);
            thread::sleep(one_sec);
        }
    }
}
