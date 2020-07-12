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
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }
    for i in 0..h {
        s[i].push('#');
    }
    s.push(vec![]);
    for _ in 0..=w {
        s[h].push('#');
    }

    let mut dp = vec![vec![(false, false); w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if s[i + 1][j] == '#' && s[i][j + 1] == '#' && s[i + 1][j + 1] == '#' {
                dp[i][j] = (false, false);
            } else {
                dp[i][j].0 = (s[i + 1][j] == '.' && !dp[i + 1][j].1)
                    || (s[i][j + 1] == '.' && !dp[i][j + 1].1)
                    || (s[i + 1][j + 1] == '.' && !dp[i + 1][j + 1].1);
                dp[i][j].1 = (s[i + 1][j] == '.' && !dp[i + 1][j].0)
                    || (s[i][j + 1] == '.' && !dp[i][j + 1].0)
                    || (s[i + 1][j + 1] == '.' && !dp[i + 1][j + 1].0);
            }
        }
    }
    if dp[0][0].0 {
        println!("First");
    } else {
        println!("Second");
    }
}
