use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    // height -> list of tx ids
    let mut chain: HashMap<u64, Vec<String>> = HashMap::new();
    let mut tip: u64 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }
        let mut it = l.splitn(2, ' ');
        match it.next().unwrap() {
            "BLOCK" => {
                let rest = it.next().unwrap();
                let mut p = rest.splitn(2, ' ');
                let h: u64 = p.next().unwrap().parse().unwrap();
                let txs: Vec<String> = p.next().unwrap().split(',').map(|s| s.to_string()).collect();
                chain.insert(h, txs);
                if h > tip { tip = h; }
            }
            "REORG" => {
                let h: u64 = it.next().unwrap().parse().unwrap();
                chain.retain(|&k, _| k <= h);
                tip = h;
            }
            "TIP_HEIGHT" => {
                println!("{}", tip);
            }
            "TX_CONFIRMATIONS" => {
                let tx = it.next().unwrap();
                let found = chain.iter().find(|(_, txs)| txs.iter().any(|t| t == tx));
                match found {
                    Some((&h, _)) => println!("{}", tip - h + 1),
                    None          => println!("UNCONFIRMED"),
                }
            }
            _ => {}
        }
    }
}
