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
        n: usize,
        m: usize,
        a: [i64; n],
        xy: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v) in xy.iter() {
        graph[u].push(v);
    }
    let mut buy = vec![INF as i64; n];
    for u in 0..n {
        for &v in graph[u].iter() {
            buy[v] = min(buy[v], min(buy[u], a[u]));
        }
    }
    println!("{}", (0..n).map(|u| a[u] - buy[u]).max().unwrap());
}
