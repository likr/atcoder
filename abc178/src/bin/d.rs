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
    }

    let mut dp = vec![0; s + 1];
    for i in 3..=s {
        dp[i] = 1;
    }
    for i in 0..=s {
        for j in 3..i {
            let k = i - j;
            dp[i] = (dp[i] + dp[k]) % M;
        }
    }
    debug!(dp);
    println!("{}", dp[s]);
}
