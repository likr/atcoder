use ac_library::*;
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
        q: usize,
        k: usize,
        query: [(char, usize); q],
    }
    let f = ModInt998244353::new;
    let mut dp = vec![f(0); k + 1];
    dp[0] = f(1);
    for i in 0..q {
        let (t, x) = query[i];
        if t == '+' {
            for j in (x..=k).rev() {
                dp[j] = dp[j] + dp[j - x];
            }
        } else {
            for j in x..=k {
                dp[j] = dp[j] - dp[j - x];
            }
        }
        println!("{}", dp[k]);
    }
}
