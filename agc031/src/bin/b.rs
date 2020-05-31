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
        c: [usize; n],
    }
    let mut dp = vec![0usize; n + 1];
    dp[0] = 1;
    let mut last_index = HashMap::new();
    for i in 1..=n {
        let ci = c[i - 1];
        if let Some(&j) = last_index.get(&ci) {
            if j < i - 1 {
                dp[i] = (dp[i - 1] + dp[j]) % M;
            } else {
                dp[i] = dp[i - 1];
            }
        } else {
            dp[i] = dp[i - 1];
        }
        last_index.insert(ci, i);
    }
    // eprintln!("{:?}", dp);
    // eprintln!("{:?}", last_index);
    println!("{}", dp[n]);
}
