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
        mut s: Chars,
    }
    s.reverse();
    let f = ModInt1000000007::new;
    let n = s.len();
    let mut dp = vec![vec![f(0); 13]; n + 1];
    dp[0][0] = f(1);
    let mut base = 1;
    for i in 0..n {
        if s[i] == '?' {
            for c in 0..10 {
                for j in 0..13 {
                    dp[i + 1][(c * base + j) % 13] = dp[i + 1][(c * base + j) % 13] + dp[i][j];
                }
            }
        } else {
            let c = s[i] as usize - '0' as usize;
            for j in 0..13 {
                dp[i + 1][(c * base + j) % 13] = dp[i + 1][(c * base + j) % 13] + dp[i][j];
            }
        }
        base = base * 10 % 13;
    }
    println!("{}", dp[n][5]);
}
