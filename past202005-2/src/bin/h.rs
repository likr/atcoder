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
        l: usize,
        x: [usize; n],
        t: [usize; 3],
    }
    let x = x.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![INF; l + 1];
    dp[0] = 0;
    for i in 0..l {
        if x.contains(&(i + 1)) {
            dp[i + 1] = min(dp[i + 1], dp[i] + t[0] + t[2]);
        } else {
            dp[i + 1] = min(dp[i + 1], dp[i] + t[0]);
        }
        if i + 2 <= l {
            if x.contains(&(i + 2)) {
                dp[i + 2] = min(dp[i + 2], dp[i] + t[0] + t[1] + t[2]);
            } else {
                dp[i + 2] = min(dp[i + 2], dp[i] + t[0] + t[1]);
            }
        }
        if i + 4 <= l {
            if x.contains(&(i + 4)) {
                dp[i + 4] = min(dp[i + 4], dp[i] + t[0] + 3 * t[1] + t[2]);
            } else {
                dp[i + 4] = min(dp[i + 4], dp[i] + t[0] + 3 * t[1]);
            }
        }
    }
    let mut result = dp[l];
    result = min(result, dp[l - 1] + (t[0] + t[1]) / 2);
    if 2 <= l {
        result = min(result, dp[l - 2] + (t[0] + 3 * t[1]) / 2);
    }
    if 3 <= l {
        result = min(result, dp[l - 3] + (t[0] + 5 * t[1]) / 2);
    }
    println!("{}", result);
}
