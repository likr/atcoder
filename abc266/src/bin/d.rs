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
        txa: [(usize, usize, usize); n],
    }
    let mut t0 = 0;
    let mut dp = vec![vec![None; 5]; n + 1];
    dp[0][0] = Some(0);
    for i in 0..n {
        let (ti, xi, ai) = txa[i];
        for j in 0..5 {
            if let Some(v) = dp[i][j] {
                for k in 0..5 {
                    if max(j, k) - min(j, k) <= ti - t0 {
                        let new_v = v + if k == xi { ai } else { 0 };
                        dp[i + 1][k] = Some(if let Some(c) = dp[i + 1][k] {
                            max(new_v, c)
                        } else {
                            new_v
                        });
                    }
                }
            }
        }
        t0 = ti;
    }
    println!(
        "{}",
        (0..5)
            .filter(|&j| dp[n][j].is_some())
            .map(|j| dp[n][j].unwrap())
            .max()
            .unwrap()
    );
}
