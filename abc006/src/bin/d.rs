use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;

    for &ci in &c {
        let k = dp.lower_bound(&ci);
        dp[k] = ci;
    }

    let mut count = 0;
    for i in 1..=n {
        if dp[i] != INF {
            count = i;
        }
    }
    println!("{}", n - count);
}
