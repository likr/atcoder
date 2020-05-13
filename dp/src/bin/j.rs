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

fn index(x: usize, y: usize, z: usize) -> usize {
    (x * 301 + y) * 301 + z
}

fn rec(dp: &mut Vec<f64>, x: usize, y: usize, z: usize, n: usize) -> f64 {
    let k = index(x, y, z);
    if dp[k] > 0. {
        return dp[k];
    }
    if x == 0 && y == 0 && z == 0 {
        return 0.;
    }

    let mut s = 0.;
    if x > 0 {
        s += rec(dp, x - 1, y, z, n) * x as f64;
    }
    if y > 0 {
        s += rec(dp, x + 1, y - 1, z, n) * y as f64;
    }
    if z > 0 {
        s += rec(dp, x, y + 1, z - 1, n) * z as f64;
    }
    s += n as f64;
    s /= (x + y + z) as f64;
    dp[k] = s;
    s
}

fn main() {
    input! {
      n: usize,
      a: [Usize1; n],
    }
    let mut count = vec![0; 3];
    for &ai in &a {
        count[ai] += 1;
    }

    let mut dp = vec![0.; 301 * 301 * 301];
    println!("{}", rec(&mut dp, count[0], count[1], count[2], n));
}
