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
const M: usize = 998244353;

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
        k: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![HashSet::new(); n];
    for &(u, v) in uv.iter() {
        graph[u].insert(v);
        graph[v].insert(u);
    }
    let mut dp = vec![vec![0; n]; k];
    for u in 1..n {
        if !graph[0].contains(&u) {
            dp[0][u] = 1;
        }
    }
    for i in 1..k {
        let mut s = 0;
        for u in 0..n {
            s = (s + dp[i - 1][u]) % M;
        }
        for u in 0..n {
            dp[i][u] = (s + M - dp[i - 1][u]) % M;
            for &v in graph[u].iter() {
                dp[i][u] = (dp[i][u] + M - dp[i - 1][v]) % M;
            }
        }
    }
    println!("{}", dp[k - 1][0]);
}
