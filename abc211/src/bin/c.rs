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
        s: Chars,
    }
    let n = s.len();
    let target = "chokudai".chars().collect::<Vec<_>>();
    let m = target.len();
    let mut dp = vec![vec![0; n + 1]; m];
    for i in (0..n).rev() {
        dp[m - 1][i] = dp[m - 1][i + 1];
        if s[i] == target[m - 1] {
            dp[m - 1][i] += 1;
        }
    }
    for j in (1..m).rev() {
        for i in (0..n).rev() {
            dp[j - 1][i] = dp[j - 1][i + 1];
            if s[i] == target[j - 1] {
                dp[j - 1][i] = (dp[j - 1][i] + dp[j][i]) % M;
            }
        }
    }
    println!("{}", dp[0][0]);
}
