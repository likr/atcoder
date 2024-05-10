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
        a: [usize; n],
        b: [usize; n],
    }
    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&i| a[i]);
    let m = ModInt998244353::new;
    let mut dp = vec![vec![m(0); 5001]; n + 1];
    dp[0][0] = m(1);
    let mut ans = m(0);
    for k in 0..n {
        let i = indices[k];
        for j in 0..=5000 {
            dp[k + 1][j] = dp[k + 1][j] + dp[k][j];
            if j + b[i] <= 5000 {
                dp[k + 1][j + b[i]] = dp[k + 1][j + b[i]] + dp[k][j];
                if j + b[i] <= a[i] {
                    ans += dp[k][j];
                }
            }
        }
    }
    println!("{}", ans);
}
