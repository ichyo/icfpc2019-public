use jsonrpc_client_http::{HttpHandle, HttpTransport};
use serde::{Deserialize, Serialize};
use log::info;
use serde_json::Value;


use crate::models::*;
use crate::parse::{read_puzzle, read_task};
use crate::puzzle::solve_puzzle;
use crate::solve::solve_small_while;
use chrono::prelude::*;
use std::collections::HashMap;
use std::{thread, time};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockChainInfo {
    pub block: usize,
    pub block_subs: usize,
    pub block_ts: f64,
    pub total_subs: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MiningInfo {
    pub block: usize,
    //excluded: Vec<usize>,
    pub puzzle: String,
    pub task: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockInfo {
    pub block: usize,
    pub block_ts: f64,
    pub puzzle: String,
    pub task: String,
    pub balances: HashMap<usize, usize>,
}

impl BlockInfo {
    pub fn time(&self) -> DateTime<Local> {
        Local.timestamp(self.block_ts as i64, 0)
    }
}

jsonrpc_client!(pub struct LambdaClient {
    pub fn getblockchaininfo(&mut self) -> RpcRequest<BlockChainInfo>;
    pub fn getmininginfo(&mut self) -> RpcRequest<MiningInfo>;
    pub fn getblockinfo(&mut self, block: usize) -> RpcRequest<BlockInfo>;
    pub fn submit(&mut self, block: usize, task_sol_path: &str, pazzle_sol_path: &str) -> RpcRequest<Value>;
});

pub struct Client {
    api: LambdaClient<HttpHandle>,
    last_block: usize,
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
        Client {
            api: client,
            last_block: 0,
        }
    }

    pub fn latest_block(&mut self) -> Option<usize> {
        match self.api.getmininginfo().call() {
            Ok(m) => Some(m.block),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    pub fn get_block_info(&mut self, bucket: usize) -> Option<BlockInfo> {
        match self.api.getblockinfo(bucket).call() {
            Ok(m) => Some(m),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    pub fn submit_latest(&mut self) {
        if let Some(block) = self.latest_block() {
            if block == self.last_block {
                return;
            }
            info!("Start {}", block);
            self.last_block = block;
            if self.generate_solution(block) {
                match self
                    .api
                    .submit(
                        block,
                        &format!("./mining/{}-task.sol", block),
                        &format!("./mining/{}-puzzle.desc", block),
                    )
                    .call()
                {
                    Ok(value) => eprintln!("{:?}", value),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }

    pub fn generate_solution(&mut self, block: usize) -> bool {
        let blockinfo = match self.api.getblockinfo(block).call() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        };

        let puzzle = read_puzzle(&blockinfo.puzzle);
        let task = read_task(&blockinfo.task);
        info!("solving puzzle");
        let puzzle_answer = solve_puzzle(puzzle);
        info!("solving task");
        let task_answer = solve_small_while(task, std::time::Duration::from_secs(300));
        info!("dumping");

        self.dump_task_answer(block, task_answer);
        if let Some(puzzle_answer) = puzzle_answer {
            self.dump_puzzle_answer(block, puzzle_answer);
            true
        } else {
            false
        }
    }

    fn dump_task_answer(&self, block: usize, answer: Commands) {
        let content = format!("{}", answer);
        match self.dump_file(&format!("./mining/{}-task.sol", block), &content) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    fn dump_puzzle_answer(&self, block: usize, answer: Task) {
        let content = format!("{}", answer);
        match self.dump_file(&format!("./mining/{}-puzzle.desc", block), &content) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    fn dump_file(&self, path: &str, content: &str) -> std::io::Result<()> {
        std::fs::write(path, content)
    }

    pub fn execute(&mut self) {
        loop {
            self.submit_latest();
            thread::sleep(time::Duration::from_secs(10));
        }
    }
}
