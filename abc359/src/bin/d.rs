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
    let f = ModInt998244353::new;
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut patterns = vec![];
    let mut next = vec![];
    for x in 0..1 << k {
        let mut pattern = vec![];
        for i in 0..k {
            if x & 1 << i > 0 {
                pattern.push('A');
            } else {
                pattern.push('B');
            }
        }
        patterns.push(pattern);
        next.push(vec![(x >> 1), (x >> 1) | (1 << (k - 1))]);
    }
    let m = patterns.len();
    let mut is_palindrome = vec![true; m];
    for i in 0..m {
        for j in 0..k {
            if patterns[i][j] != patterns[i][k - 1 - j] {
                is_palindrome[i] = false;
            }
        }
    }
    let mut dp = vec![vec![f(0); m]; n];
    for j in 0..m {
        if !is_palindrome[j] && (0..k).all(|i| s[i] == patterns[j][i] || s[i] == '?') {
            dp[k - 1][j] = f(1);
        }
    }
    for i in k..n {
        for j in 0..m {
            for &u in next[j].iter() {
                if !is_palindrome[u] && (s[i] == patterns[u][k - 1] || s[i] == '?') {
                    dp[i][u] = dp[i][u] + dp[i - 1][j]
                }
            }
        }
    }
    let mut ans = f(0);
    for j in 0..m {
        ans = ans + dp[n - 1][j];
    }
    println!("{}", ans);
}
