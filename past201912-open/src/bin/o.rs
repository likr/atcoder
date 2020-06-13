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
        mut a: [[usize; 6]; n],
    }
    let m = 6 * n;
    let mut b = vec![];
    for i in 0..n {
        for j in 0..6 {
            b.push((a[i][j], i, j));
        }
    }
    b.sort();
    for k in 0..m {
        let (_, i, j) = b[k];
        a[i][j] = k;
    }

    let mut dp = vec![0.; m + 1];
    let mut dice = vec![1.; n];
    let mut dice_max = 1.;
    for k in (0..m).rev() {
        let (_, i, _) = b[k];
        dp[k] = dice_max;
        dice[i] += dp[k] / 6.;
        if dice[i] > dice_max {
            dice_max = dice[i];
        }
    }

    println!("{}", dice_max);
}
