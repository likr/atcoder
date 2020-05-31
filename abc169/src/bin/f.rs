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
const M: usize = 998244353;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![0usize; s + 1];
    dp[0] = 1;
    for i in 0..n {
        let ai = a[i];
        for j in (0..=s).rev() {
            if j >= ai {
                dp[j] = (dp[j - ai] + 2 * dp[j]) % M;
            } else {
                dp[j] = (2 * dp[j]) % M;
            }
        }
    }
    // for i in 1..=n {
    //     eprintln!("{:?}", dp[i]);
    // }
    println!("{}", dp[s]);
}
