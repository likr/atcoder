use itertools::Itertools;
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
        xc: [(i64, usize); n],
    }
    let mut c_x = HashMap::new();
    for i in 0..n {
        let (xi, ci) = xc[i];
        c_x.entry(ci).or_insert(vec![]).push(xi);
    }
    c_x.insert(0, vec![0]);
    let m = c_x.len();
    let mut ranges = c_x
        .iter()
        .sorted_by_key(|(&ci, _)| ci)
        .map(|(_, xs)| (*xs.iter().min().unwrap(), *xs.iter().max().unwrap()))
        .collect::<Vec<_>>();
    ranges.push((0, 0));

    let mut dp = vec![[0; 2]; m + 1];
    for i in 1..=m {
        let (left0, right0) = ranges[i - 1];
        let (left, right) = ranges[i];
        dp[i][0] = min(
            dp[i - 1][0] + (left0 - right).abs() + (right - left),
            dp[i - 1][1] + (right0 - right).abs() + (right - left),
        );
        dp[i][1] = min(
            dp[i - 1][0] + (left0 - left).abs() + (right - left),
            dp[i - 1][1] + (right0 - left).abs() + (right - left),
        );
    }
    println!("{}", min(dp[m][0], dp[m][1]));
}
