use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }
        let mut it = l.splitn(3, '|');
        let old: u64 = it.next().unwrap().parse().unwrap();
        let actual: u64 = it.next().unwrap().parse().unwrap();
        let expected: u64 = it.next().unwrap().parse().unwrap();
        let new = (old * actual / expected).max(old / 4).min(old * 4);
        println!("{}", new);
    }
}
