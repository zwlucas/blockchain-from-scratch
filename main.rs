use std::io::{self, BufRead};

fn classify(s: &str) -> &'static str {
    let l = s.to_lowercase();
    if l.contains("permissionless") || l.contains("miner") { return "POW"; }
    if l.contains("slash") || l.contains("stake") || l.contains("eco") || l.contains("finality") { return "POS"; }
    if l.contains("trusted") || l.contains("microservice") || l.contains("leader election") { return "RAFT"; }
    if l.contains("consortium") || l.contains("institution") || l.contains("permissioned") { return "BFT"; }
    "POW"
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if !l.is_empty() { println!("{}", classify(&l)); }
    }
}
