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
        mut n: Chars,
    }
    n.reverse();
    n.push('0');
    let n = n
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    let m = n.len();
    let mut dp = vec![vec![(0, 0); 10]; m];
    dp[0][1] = (1, 1);
    for i in 1..m {
        for j in 0..10 {
            if j == 1 {
                dp[i][j].0 += 10usize.pow(i as u32);
            }
            for k in 0..10 {
                dp[i][j].0 += dp[i - 1][k].0;
            }

            if j == 1 {
                let mut s = 1;
                for k in 0..i {
                    s += n[k] * 10usize.pow(k as u32);
                }
                dp[i][j].1 += s;
            }
            if j < n[i] {
                dp[i][j].1 = dp[i][j].0;
            } else if j == n[i] {
                for k in 0..=n[i - 1] {
                    dp[i][j].1 += dp[i - 1][k].1;
                }
            }
        }
        eprintln!("{:?}", dp[i]);
    }
    println!("{}", dp[m - 1][0].1);
}
