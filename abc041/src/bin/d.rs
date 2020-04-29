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

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(xi, yi) in &xy {
        graph[xi].push(yi);
    }

    let m = 2usize.pow(n as u32);
    let mut dp = vec![0usize; m];
    dp[0] = 1;
    for x in 1..m {
        let mut s = 0;
        for u in 0..n {
            if x & 1 << u == 0 {
                continue;
            }
            if graph[u].iter().all(|&v| x & 1 << v == 0) {
                s += dp[x ^ 1 << u];
            }
        }
        dp[x] = s;
    }

    println!("{}", dp[m - 1]);
}
