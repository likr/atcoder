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
        s: Chars,
    }
    let f = ModInt998244353::new;
    let n = s.len();
    let mut dp = vec![vec![f(0); n + 1]; n + 1];
    dp[0][0] = f(1);
    for i in 0..n {
        for j in 0..=n {
            if (s[i] == '(' || s[i] == '?') && j + 1 <= n {
                dp[i + 1][j + 1] = dp[i + 1][j + 1] + dp[i][j];
            }
            if (s[i] == ')' || s[i] == '?') && j > 0 {
                dp[i + 1][j - 1] = dp[i + 1][j - 1] + dp[i][j];
            }
        }
    }
    println!("{}", dp[n][0]);
}
