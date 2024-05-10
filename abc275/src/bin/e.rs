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

fn inv(a: usize) -> usize {
    let m = M as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut dp = vec![vec![0; n + 1]; k + 1];
    dp[0][0] = 1;
    for i in 0..k {
        for j in 0..n {
            for l in 1..=m {
                if j + l <= n {
                    dp[i + 1][j + l] = (dp[i + 1][j + l] + dp[i][j]) % M;
                } else {
                    dp[i + 1][n - (j + l - n)] = (dp[i + 1][n - (j + l - n)] + dp[i][j]) % M;
                }
            }
        }
        dp[i + 1][n] = (dp[i + 1][n] + m * dp[i][n]) % M;
    }
    let s = dp[k][n];
    let mut t = 0;
    for i in 1..=n {
        t = (t + dp[k][i]) % M;
    }
    println!("{}", s * inv(t) % M);
}
