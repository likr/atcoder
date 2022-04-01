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
        s: Chars,
    }
    let s = s
        .into_iter()
        .map(|c| (c as u8 - 'A' as u8) as usize)
        .collect::<Vec<_>>();
    let mut dp = vec![vec![vec![0usize; 10]; 1024]; n];
    dp[0][1 << s[0]][s[0]] = 1;
    for i in 1..n {
        let si = s[i];
        for j in 1..1024 {
            for k in 0..10 {
                // not use
                dp[i][j][k] = dp[i - 1][j][k];
            }
        }
        for j in 1..1024 {
            if j & 1 << si > 0 {
                // use or not use
                dp[i][j][si] = dp[i][j][si] * 2 % M;
            }
        }
        for j in 1..1024 {
            if j & 1 << si == 0 {
                for k in 0..10 {
                    if k != si {
                        // use
                        dp[i][j | 1 << si][si] = (dp[i][j | 1 << si][si] + dp[i][j][k]) % M;
                    }
                }
            }
        }
        // use
        dp[i][1 << si][si] = (dp[i][1 << si][si] + 1) % M;
    }
    let mut result = 0;
    for j in 1..1024 {
        for k in 0..10 {
            result = (result + dp[n - 1][j][k]) % M;
        }
    }
    println!("{}", result);
}
