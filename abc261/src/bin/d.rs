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
        x: [usize; n],
        cy: [(Usize1, usize); m],
    }
    let mut y = vec![0; n];
    for &(ci, yi) in cy.iter() {
        y[ci] = yi;
    }
    let mut dp = vec![vec![None; n + 1]; n + 1];
    dp[0][0] = Some(0usize);
    for i in 0..n {
        for j in 0..n {
            if let Some(p) = dp[i][j] {
                if let Some(q) = dp[i + 1][0] {
                    dp[i + 1][0] = Some(max(p, q));
                } else {
                    dp[i + 1][0] = Some(p);
                }
                let new_p = p + x[i] + y[j];
                if let Some(q) = dp[i + 1][j + 1] {
                    dp[i + 1][j + 1] = Some(max(q, new_p));
                } else {
                    dp[i + 1][j + 1] = Some(new_p);
                }
            }
        }
    }
    let mut ans = 0;
    for j in 0..=n {
        if let Some(p) = dp[n][j] {
            ans = max(ans, p);
        }
    }
    println!("{}", ans);
}
