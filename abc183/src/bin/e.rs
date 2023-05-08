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
        h: usize,
        w: usize,
        t: [Chars; h],
    }
    let mut s = vec![vec!['#'; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            s[i + 1][j + 1] = t[i][j];
        }
    }
    let mut count1 = vec![vec![0; w + 1]; h + 1];
    let mut count2 = vec![vec![0; w + 1]; h + 1];
    let mut count3 = vec![vec![0; w + 1]; h + 1];
    let mut dp = vec![vec![0; w + 1]; h + 1];
    dp[1][1] = 1;
    for i in 1..=h {
        for j in 1..=w {
            if i == 1 && j == 1 {
                continue;
            }
            if s[i][j] == '.' {
                count1[i][j] = (count1[i][j - 1] + dp[i][j - 1]) % M;
                count2[i][j] = (count2[i - 1][j] + dp[i - 1][j]) % M;
                count3[i][j] = (count3[i - 1][j - 1] + dp[i - 1][j - 1]) % M;
                dp[i][j] = (count1[i][j] + count2[i][j] + count3[i][j]) % M;
            }
        }
    }
    println!("{}", dp[h][w]);
}
