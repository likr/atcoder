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
    let f = ModInt998244353::new;
    input! {
        n: usize,
    }
    let mut dp = vec![vec![f(0); 10]; n];
    for j in 1..10 {
        dp[0][j] = f(1);
    }
    for i in 1..n {
        for j in 1..10usize {
            for d in [!0, 0, 1] {
                let j2 = j.wrapping_add(d);
                if 1 <= j2 && j2 <= 9 {
                    dp[i][j2] = dp[i][j2] + dp[i - 1][j];
                }
            }
        }
    }
    let mut ans = f(0);
    for j in 1..10 {
        ans = ans + dp[n - 1][j];
    }
    println!("{}", ans);
}
