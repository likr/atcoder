use ac_library::*;
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
        a: [[usize; 6]; n],
    }
    let m = ModInt1000000007::new;
    let mut dp = vec![m(0); n];
    for j in 0..6 {
        dp[0] += m(a[0][j]);
    }
    for i in 1..n {
        for j in 0..6 {
            dp[i] = dp[i] + dp[i - 1] * m(a[i][j]);
        }
    }
    println!("{}", dp[n - 1]);
}
