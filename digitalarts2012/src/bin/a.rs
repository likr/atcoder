#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    let stdin = std::io::stdin();
    let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
    let mut next = move || -> String {
        bytes
            .by_ref()
            .map(|r| r.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect()
    };

    let mut s = vec![];
    let n;
    loop {
        let si = next();
        if let Some(m) = si.parse::<usize>().ok() {
            n = m;
            break;
        } else {
            s.push(si.chars().collect::<Vec<_>>());
        }
    }
    let mut t = vec![];
    for _ in 0..n {
        t.push(next().chars().collect::<Vec<_>>());
    }

    let m = s.len();
    let mut result = vec![];
    for i in 0..m {
        let l = s[i].len();
        if (0..n).any(|j| l == t[j].len() && (0..l).all(|k| t[j][k] == '*' || s[i][k] == t[j][k])) {
            result.push("*".repeat(l));
        } else {
            result.push(s[i].iter().collect::<String>());
        }
    }
    println!("{}", result.join(" "));
}
