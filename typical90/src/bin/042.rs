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
        k: usize,
    }
    if k % 9 != 0 {
        println!("0");
        return;
    }
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 0..k {
        for j in 1..10 {
            if i + j <= k {
                dp[i + j] = (dp[i + j] + dp[i]) % M;
            }
        }
    }
    println!("{}", dp[k]);
}
