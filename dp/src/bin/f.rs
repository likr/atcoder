use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
      s: Chars,
      t: Chars,
    }
    let ns = s.len();
    let nt = t.len();
    let mut dp = vec![vec![0; nt + 1]; ns + 1];
    let mut parent = vec![vec![(0, 0); nt + 1]; ns + 1];
    for i in 1..ns {
        parent[i][0] = (i - 1, 0);
    }
    for j in 1..nt {
        parent[0][j] = (0, j - 1);
    }

    for i in 1..=ns {
        for j in 1..=nt {
            if dp[i - 1][j] > dp[i][j - 1] {
                dp[i][j] = dp[i - 1][j];
                parent[i][j] = (i - 1, j);
            } else {
                dp[i][j] = dp[i][j - 1];
                parent[i][j] = (i, j - 1);
            }
            if s[i - 1] == t[j - 1] {
                if dp[i - 1][j - 1] + 1 > dp[i][j] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    parent[i][j] = (i - 1, j - 1);
                }
            }
        }
    }
    let mut result = vec![];
    let mut i = ns;
    let mut j = nt;
    while i != 0 && j != 0 {
        let (i2, j2) = parent[i][j];
        if i2 < i && j2 < j {
            result.push(s[i2]);
        }
        i = i2;
        j = j2;
    }
    result.reverse();
    for &c in &result {
        print!("{}", c);
    }
    println!("");
}
