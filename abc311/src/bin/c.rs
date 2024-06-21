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
        a: [Usize1; n],
    }
    let m = 20;
    let mut dp = vec![vec![0; n]; m + 1];
    for u in 0..n {
        dp[0][u] = a[u];
    }
    for j in 0..m {
        for i in 0..n {
            dp[j + 1][i] = dp[j][dp[j][i]];
        }
    }
    debug!(dp[m]);
    let s = dp[m][0];
    let mut u = s;
    let mut ans = vec![];
    loop {
        ans.push(format!("{}", u + 1));
        u = a[u];
        if u == s {
            break;
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.join(" "));
}
