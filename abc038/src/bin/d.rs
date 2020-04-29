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
        mut hw: [(usize, usize); n],
    }
    hw.sort_by_key(|&(hi, wi)| (hi, Reverse(wi)));

    let w_max = hw.iter().map(|&(_, wi)| wi).max().unwrap() + 1;
    let mut m = 1;
    while m < w_max {
        m *= 2;
    }
    let mut bit = vec![0; m + 1];
    let mut dp = vec![0; n];
    for i in 0..n {
        let (_, wi) = hw[i];
        let mut x = wi;
        for j in 0.. {
            if x & 1 << j > 0 {
                dp[i] = max(dp[i], bit[x]);
                x -= 1 << j;
                if x == 0 {
                    break;
                }
            }
        }
        dp[i] += 1;

        let mut x = wi + 1;
        for j in 0.. {
            if x & 1 << j == 0 {
                continue;
            }
            bit[x] = max(bit[x], dp[i]);
            x += 1 << j;
            if x > m {
                break;
            }
        }
        // eprintln!("{:?}", dp);
        // eprintln!("{:?}", bit);
    }
    println!("{}", dp.iter().max().unwrap());
}
