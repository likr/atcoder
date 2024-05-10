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
        a: [usize; n],
    }
    let mut dp = vec![vec![(None, None); m + 1]; n];
    if a[0] <= m {
        dp[0][a[0]].0 = Some(0usize);
    }
    dp[0][0].1 = Some(1usize);
    for i in 1..n {
        for j in 0..=m {
            if let Some(c) = dp[i - 1][j].0 {
                if let Some(d) = dp[i][j].1 {
                    dp[i][j].1 = Some(min(d, c + 1));
                } else {
                    dp[i][j].1 = Some(c + 1);
                }
                if j + a[i] <= m {
                    if let Some(d) = dp[i][j + a[i]].0 {
                        dp[i][j + a[i]].0 = Some(min(d, c));
                    } else {
                        dp[i][j + a[i]].0 = Some(c);
                    }
                }
            }
            if let Some(c) = dp[i - 1][j].1 {
                if let Some(d) = dp[i][j].1 {
                    dp[i][j].1 = Some(min(d, c));
                } else {
                    dp[i][j].1 = Some(c);
                }
                if j + a[i] <= m {
                    if let Some(d) = dp[i][j + a[i]].0 {
                        dp[i][j + a[i]].0 = Some(min(d, c));
                    } else {
                        dp[i][j + a[i]].0 = Some(c);
                    }
                }
            }
        }
    }
    for j in 1..=m {
        let mut result = vec![];
        if let Some(c) = dp[n - 1][j].0 {
            result.push(c);
        }
        if let Some(c) = dp[n - 1][j].1 {
            result.push(c);
        }
        if result.len() == 0 {
            println!("-1");
        } else {
            result.sort();
            println!("{}", result[0]);
        }
    }
}
