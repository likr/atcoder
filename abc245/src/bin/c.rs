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
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let mut dp = vec![(false, false); n];
    dp[0] = (true, true);
    for i in 1..n {
        if dp[i - 1].0 {
            if (a[i - 1] - a[i]).abs() <= k {
                dp[i].0 = true;
            }
            if (a[i - 1] - b[i]).abs() <= k {
                dp[i].1 = true;
            }
        }
        if dp[i - 1].1 {
            if (b[i - 1] - a[i]).abs() <= k {
                dp[i].0 = true;
            }
            if (b[i - 1] - b[i]).abs() <= k {
                dp[i].1 = true;
            }
        }
    }
    if dp[n - 1].0 || dp[n - 1].1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
