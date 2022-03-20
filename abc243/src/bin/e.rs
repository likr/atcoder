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
        abc: [(Usize1, Usize1, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi, ci) in abc.iter() {
        graph[ai].push((bi, ci));
        graph[bi].push((ai, ci));
    }
    let mut d = vec![vec![INF; n]; n];
    for u in 0..n {
        for &(v, c) in graph[u].iter() {
            d[u][v] = c;
            d[v][u] = c;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] + d[k][j] < d[i][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }
    let mut result = 0;
    for &(i, j, c) in abc.iter() {
        if (0..n).any(|k| d[i][k] + d[k][j] <= c) {
            result += 1;
        }
    }
    println!("{}", result);
}
