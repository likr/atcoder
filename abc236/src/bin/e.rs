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
const INF: i64 = std::i64::MAX / 4;
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
        mut a: [i64; n],
    }
    {
        for i in 0..n {
            a[i] *= 1000;
        }
        let mut l = 0;
        let mut h = 100000000000000;
        while h - l > 1 {
            let m = (l + h) / 2;
            let mut dp = vec![vec![0; 2]; n + 1];
            for i in 0..n {
                dp[i + 1][0] = dp[i][1];
                dp[i + 1][1] = max(dp[i][0], dp[i][1]) + a[i] - m;
            }
            if max(dp[n][0], dp[n][1]) >= 0 {
                l = m;
            } else {
                h = m;
            }
        }
        println!("{}", l as f64 / 1000.);
        for i in 0..n {
            a[i] /= 1000;
        }
    }
    {
        let mut l = 0;
        let mut h = 100000000000000;
        while h - l > 1 {
            let m = (l + h) / 2;
            let mut dp = vec![vec![0; 2]; n + 1];
            for i in 0..n {
                dp[i + 1][0] = dp[i][1];
                dp[i + 1][1] = max(dp[i][0], dp[i][1]) + if a[i] >= m { 1 } else { -1 };
            }
            if max(dp[n][0], dp[n][1]) > 0 {
                l = m;
            } else {
                h = m;
            }
        }
        println!("{}", l);
    }
}
