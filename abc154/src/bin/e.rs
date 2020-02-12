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
      n: Chars,
      k: usize,
    }
    let m = n.len();
    let mut dp0 = vec![vec![0; k + 1]; m + 1];
    let mut dp1 = vec![vec![0; k + 1]; m + 1];
    dp0[0][0] = 1;
    for i in 0..m {
        let d = n[i] as usize - '0' as usize;
        for j in 0..=k {
            if d == 0 {
                dp0[i + 1][j] = dp0[i][j];
            } else {
                dp1[i + 1][j] += dp0[i][j];
            }
            dp1[i + 1][j] += dp1[i][j];
        }
        for j in 0..k {
            if d > 0 {
                dp0[i + 1][j + 1] = dp0[i][j];
                dp1[i + 1][j + 1] += (d - 1) * dp0[i][j];
            }
            dp1[i + 1][j + 1] += 9 * dp1[i][j];
        }
    }
    println!("{}", dp0[m][k] + dp1[m][k]);
}
