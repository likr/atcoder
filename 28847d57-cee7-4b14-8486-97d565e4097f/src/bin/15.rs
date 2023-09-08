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
        s: Chars,
    }
    let mut dp = vec![INF; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=3 {
            if i + j < n {
                if s[i + j] == '.' {
                    dp[i + j] = min(dp[i + j], dp[i]);
                } else {
                    dp[i + j] = min(dp[i + j], dp[i] + 1);
                }
            }
        }
    }
    println!("{}", dp[n - 1]);
}
