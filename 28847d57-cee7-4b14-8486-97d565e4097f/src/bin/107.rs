use proconio::input;
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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        s: usize,
        t: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; s];
    for &(u, v) in uv.iter() {
        graph[u].push(v);
    }
    let mut adj = vec![vec![None; t]; t];
    for u in 0..s {
        for &v1 in graph[u].iter() {
            for &v2 in graph[u].iter() {
                if v1 == v2 {
                    continue;
                }
                if let Some(u2) = adj[v1 - s][v2 - s] {
                    println!("{} {} {} {}", u + 1, v1 + 1, u2 + 1, v2 + 1);
                    return;
                } else {
                    adj[v1 - s][v2 - s] = Some(u);
                }
            }
        }
    }
    println!("-1");
}
