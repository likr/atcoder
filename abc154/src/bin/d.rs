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
      k: usize,
      p: [usize; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + p[i];
    }
    let mut result = 0;
    for i in 0..=n - k {
        let s = acc[i + k] - acc[i];
        result = max(result, s);
    }
    println!("{}", (result + k) as f64 / 2.);
}
