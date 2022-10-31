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
        s: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![(false, None); s + 1]; n + 1];
    dp[0][0] = (true, None);
    for i in 0..n {
        let (ai, bi) = ab[i];
        for j in 0..=s {
            if dp[i][j].0 {
                if j + ai <= s {
                    dp[i + 1][j + ai] = (true, Some(true));
                }
                if j + bi <= s {
                    dp[i + 1][j + bi] = (true, Some(false));
                }
            }
        }
    }
    if dp[n][s].0 {
        println!("Yes");
        let mut result = vec![];
        let mut j = s;
        for i in (0..n).rev() {
            let (ai, bi) = ab[i];
            if dp[i + 1][j].1.unwrap() {
                result.push("H");
                j -= ai;
            } else {
                result.push("T");
                j -= bi;
            }
        }
        result.reverse();
        println!("{}", result.join(""));
    } else {
        println!("No");
    }
}
