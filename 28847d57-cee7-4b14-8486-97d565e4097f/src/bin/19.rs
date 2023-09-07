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
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.reverse();
    b.reverse();
    let mut dp = vec![vec![(0, 0); m + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=m {
            if j + 1 <= m {
                if (n + m - i - j) % 2 == 0 {
                    if dp[i][j].0 + b[j] > dp[i][j + 1].0 {
                        dp[i][j + 1] = (dp[i][j].0 + b[j], dp[i][j].1);
                    }
                } else {
                    if dp[i][j].1 + b[j] > dp[i][j + 1].1 {
                        dp[i][j + 1] = (dp[i][j].0, dp[i][j].1 + b[j]);
                    }
                }
            }
            if i + 1 <= n {
                if (n + m - i - j) % 2 == 0 {
                    if dp[i][j].0 + a[i] > dp[i + 1][j].0 {
                        dp[i + 1][j] = (dp[i][j].0 + a[i], dp[i][j].1);
                    }
                } else {
                    if dp[i][j].1 + a[i] > dp[i + 1][j].1 {
                        dp[i + 1][j] = (dp[i][j].0, dp[i][j].1 + a[i]);
                    }
                }
            }
        }
    }
    println!("{}", dp[n][m].1);
}
