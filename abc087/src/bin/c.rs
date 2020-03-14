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
      n: usize,
      a: [[usize; n]; 2],
    }
    let mut s0 = vec![0; n];
    s0[0] = a[0][0];
    for i in 1..n {
        s0[i] = s0[i - 1] + a[0][i];
    }
    let mut s1 = vec![0; n];
    s1[n - 1] = a[1][n - 1];
    for i in (0..n - 1).rev() {
        s1[i] = s1[i + 1] + a[1][i];
    }
    let mut result = 0;
    for i in 0..n {
        result = max(result, s0[i] + s1[i]);
    }
    println!("{}", result);
}
