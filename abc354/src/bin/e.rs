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

fn rec(x: usize, h: usize, pairs: &HashSet<(usize, usize)>, dp: &mut Vec<Option<bool>>) -> bool {
    if let Some(ans) = dp[x] {
        ans
    } else {
        let ans = if h % 2 == 0 {
            pairs
                .iter()
                .filter(|&(i, j)| x & 1 << i > 0 && x & 1 << j > 0)
                .any(|&(i, j)| rec(x ^ (1 << i | 1 << j), h + 1, pairs, dp))
        } else {
            pairs
                .iter()
                .filter(|&(i, j)| x & 1 << i > 0 && x & 1 << j > 0)
                .all(|&(i, j)| rec(x ^ (1 << i | 1 << j), h + 1, pairs, dp))
        };
        dp[x] = Some(ans);
        ans
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut pairs = HashSet::new();
    for j in 0..n {
        let (aj, bj) = ab[j];
        for i in 0..j {
            let (ai, bi) = ab[i];
            if ai == aj || bj == bi {
                pairs.insert((i, j));
            }
        }
    }
    let mut dp = vec![None; 1 << n];
    if rec((1 << n) - 1, 0, &pairs, &mut dp) {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
