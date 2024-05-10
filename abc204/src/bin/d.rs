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
        t: [usize; n],
    }
    let mut s = 0;
    for i in 0..n {
        s += t[i];
    }
    let mut ng = 0;
    let mut ok = s;
    while ok - ng > 1 {
        let m = (ng + ok) / 2;
        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true;
        for i in 0..n {
            for j in 0..m {
                dp[i + 1][j] |= dp[i][j];
                if j + t[i] <= m {
                    dp[i + 1][j + t[i]] |= dp[i][j];
                }
            }
        }
        let v1 = (0..=m).filter(|&j| dp[n][j]).max().unwrap();
        let v2 = s - v1;
        if max(v1, v2) <= m {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}
