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
        a: Usize1,
        b: Usize1,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(xi, yi) in &xy {
        graph[xi].push(yi);
        graph[yi].push(xi);
    }
    let mut dp = vec![vec![0; n]; n];
    dp[0][a] = 1;
    for i in 1..n {
        for u in 0..n {
            if dp[i - 1][u] > 0 {
                for &v in &graph[u] {
                    dp[i][v] = (dp[i][v] + dp[i - 1][u]) % M;
                }
            }
        }
        if dp[i][b] > 0 {
            println!("{}", dp[i][b]);
            return;
        }
    }
}
