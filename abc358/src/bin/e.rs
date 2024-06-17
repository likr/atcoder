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
        k: usize,
        c: [usize; 26],
    }
    let f = ModInt998244353::new;
    let mut fact = vec![f(1); k + 1];
    let mut fact_inv = vec![f(1); k + 1];
    for i in 0..k {
        fact[i + 1] = fact[i] * f(i + 1);
        fact_inv[i + 1] = fact[i + 1].inv();
    }
    let mut dp = vec![vec![f(0); k + 1]; 27];
    dp[0][0] = f(1);
    for i in 0..26 {
        for j in 0..=k {
            for l in 0..=c[i] {
                if j + l <= k {
                    dp[i + 1][j + l] =
                        dp[i + 1][j + l] + dp[i][j] * fact[j + l] * fact_inv[j] * fact_inv[l];
                }
            }
        }
    }
    let mut ans = f(0);
    for j in 1..=k {
        ans = ans + dp[26][j];
    }
    println!("{}", ans);
}
