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

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: [Chars; n],
    }
    let m = s.len();
    let mut candidates = vec![vec![]; m];
    for i in 0..n {
        for j in 0..m {
            if j + t[i].len() > m {
                break;
            }
            if (0..t[i].len()).all(|k| s[j + k] == t[i][k]) {
                candidates[j].push(i);
            }
        }
    }

    let mut dp = vec![0; m + 1];
    dp[m] = 1;
    for j in (0..m).rev() {
        for &i in &candidates[j] {
            eprintln!("{} {}", i, j);
            let l = t[i].len();
            dp[j] = (dp[j] + dp[j + l]) % M;
        }
    }
    println!("{}", dp[0]);
}
