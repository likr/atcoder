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
    let mut dp = vec![vec![f(0); n + 1]; n + 1];
    dp[1][1] = f(1);
    for i in 2..=n {
        let mut s = f(0);
        let mut t = f(1);
        for j in (1..i).rev() {
            t = t / f(2);
            s = s + dp[i - 1][j] * t;
        }
        let p = f(2).pow(i as u64);
        debug!(s, p);
        dp[i][i] = s * p / (p - f(1));
        dp[i][1] = dp[i][i] / f(2);
        for j in 2..i {
            dp[i][j] = dp[i][j - 1] / f(2) + dp[i - 1][j - 1] / f(2);
        }
    }
    let mut ans = vec![];
    for i in 1..=n {
        ans.push(format!("{}", dp[n][i]));
    }
    println!("{}", ans.join(" "));
}
