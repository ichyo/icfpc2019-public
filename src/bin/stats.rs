use icfpc::mine::Client;
use std::collections::HashMap;

fn main() {
    let mut client = Client::new();
    let team_id = 42;
    let latest_block = client.latest_block().unwrap();
    let mut last_balances = HashMap::new();
    for b in latest_block - 10..=latest_block {
        let info = client.get_block_info(b).unwrap();
        let mut increses = Vec::new();
        let my_last = last_balances.get(&team_id).unwrap_or(&0);
        let my_increase = info.balances.get(&team_id).unwrap_or(my_last) - my_last;
        for (&k, &v) in &info.balances {
            increses.push((k, v - last_balances.get(&k).unwrap_or(&0)));
            last_balances.insert(k, v);
        }
        increses.sort_by_key(|(_, v)| *v);
        increses.reverse();
        let rank = increses
            .iter()
            .enumerate()
            .find(|(_, (k, _))| *k == team_id)
            .map(|(r, _)| r + 1)
            .unwrap_or(0);
        println!("{}({}): +{} (rank {})", b, info.time(), my_increase, rank);
        for (k, v) in increses.iter().take(3) {
            println!("\t{}: {}", k, v);
        }
    }
}