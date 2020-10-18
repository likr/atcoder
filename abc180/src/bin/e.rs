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
const INF: i64 = std::i64::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn rec(dp: &mut Vec<Vec<i64>>, distance: &Vec<Vec<i64>>, state: usize, u: usize, n: usize) {
    if dp[state][u] != INF {
        return;
    }
    for v in 0..n {
        if state & 1 << v == 0 {
            rec(dp, distance, state | 1 << v, v, n);
            dp[state][u] = min(dp[state][u], dp[state | 1 << v][v] + distance[v][u]);
        }
    }
}

fn main() {
    input! {
        n: usize,
        xyz: [(i64, i64, i64); n],
    }
    let mut distance = vec![vec![0; n]; n];
    for i in 0..n {
        let (a, b, c) = xyz[i];
        for j in 0..n {
            let (p, q, r) = xyz[j];
            distance[i][j] = (p - a).abs() + (q - b).abs() + max(0, r - c);
        }
        distance[i][i] = INF;
    }
    debug!(distance);

    let mut dp = vec![vec![INF; n]; 2usize.pow(n as u32)];
    let x = dp.len() - 1;
    for i in 0..n {
        dp[x][i] = INF / 4;
    }
    dp[x][0] = 0;

    rec(&mut dp, &distance, 0, 0, n);
    for s in 0..dp.len() {
        debug!(s, dp[s]);
    }
    println!("{}", dp[0][0]);
}
