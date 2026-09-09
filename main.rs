use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    // utxos: id -> (amount, owner, spent)
    let mut utxos: HashMap<String, (u64, String, bool)> = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }
        let tokens: Vec<&str> = l.splitn(2, ' ').collect();
        match tokens[0] {
            "MINT" => {
                let parts: Vec<&str> = tokens[1].splitn(3, ' ').collect();
                let id = parts[0].to_string();
                let amount: u64 = parts[1].parse().unwrap();
                let owner = parts[2].to_string();
                utxos.insert(id, (amount, owner, false));
            }
            "TX" => {
                let mut sides = tokens[1].splitn(2, " -> ");
                let in_str  = sides.next().unwrap();
                let out_str = sides.next().unwrap();

                // Parse inputs: id:owner,...
                let inputs: Vec<(&str, &str)> = in_str.split(',').map(|s| {
                    let mut p = s.splitn(2, ':');
                    (p.next().unwrap(), p.next().unwrap())
                }).collect();

                // Parse outputs: id:amount:owner,...
                let outputs: Vec<(&str, u64, &str)> = out_str.split(',').map(|s| {
                    let mut p = s.splitn(3, ':');
                    let id  = p.next().unwrap();
                    let amt: u64 = p.next().unwrap().parse().unwrap();
                    let own = p.next().unwrap();
                    (id, amt, own)
                }).collect();

                // Validate all inputs before mutating anything
                let mut in_total: u64 = 0;
                let mut err: Option<&str> = None;
                for &(id, claimed_owner) in &inputs {
                    match utxos.get(id) {
                        Some((_, _, true))                   => { err = Some("double_spend"); break; }
                        Some((amt, owner, false)) if owner != claimed_owner => { err = Some("wrong_owner"); break; }
                        Some((amt, _, false))                => { in_total += amt; }
                        None                                 => { err = Some("double_spend"); break; }
                    }
                }
                if err.is_none() {
                    let out_total: u64 = outputs.iter().map(|o| o.1).sum();
                    if out_total > in_total { err = Some("insufficient"); }
                }

                if let Some(e) = err {
                    println!("BAD {}", e);
                } else {
                    let out_total: u64 = outputs.iter().map(|o| o.1).sum();
                    // Commit: mark inputs spent, add outputs
                    for &(id, _) in &inputs {
                        utxos.get_mut(id).unwrap().2 = true;
                    }
                    for &(id, amt, own) in &outputs {
                        utxos.insert(id.to_string(), (amt, own.to_string(), false));
                    }
                    println!("OK fee={}", in_total - out_total);
                }
            }
            "BALANCE" => {
                let owner = tokens[1];
                let bal: u64 = utxos.values()
                    .filter(|(_, o, spent)| o == owner && !spent)
                    .map(|(amt, _, _)| amt)
                    .sum();
                println!("{}", bal);
            }
            _ => {}
        }
    }
}
