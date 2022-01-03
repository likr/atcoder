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
        mut s: usize,
        ab: [(usize, usize); n],
    }
    let mut w = vec![];
    for i in 0..n {
        let (ai, bi) = ab[i];
        w.push(max(ai, bi) - min(ai, bi));
        if min(ai, bi) > s {
            println!("Impossible");
            return;
        }
        s -= min(ai, bi);
    }
    let mut dp = vec![vec![None; s + 1]; n + 1];
    dp[0][0] = Some(INF);
    for i in 0..n {
        let wi = w[i];
        for j in 0..=s {
            if wi <= j && dp[i][j - wi].is_some() {
                dp[i + 1][j] = Some(j - wi);
            } else if dp[i][j].is_some() {
                dp[i + 1][j] = Some(j);
            }
        }
    }
    if dp[n][s].is_none() {
        println!("Impossible");
    } else {
        let mut sol = vec![];
        let mut t = s;
        for i in (0..n).rev() {
            let (ai, bi) = ab[i];
            let d = dp[i + 1][t].unwrap();
            if t != d {
                sol.push(if ai > bi { "A" } else { "B" });
                t = d;
            } else {
                sol.push(if ai > bi { "B" } else { "A" });
            }
        }
        sol.reverse();
        println!("{}", sol.join(""));
    }
}
