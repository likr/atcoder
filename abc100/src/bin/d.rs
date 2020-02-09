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
const INF: i64 = std::i64::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
      n: usize,
      m: usize,
      mut xyz: [(i64, i64, i64); n],
    }
    let mut result = -INF;
    for f1 in vec![1, -1] {
        for f2 in vec![1, -1] {
            for f3 in vec![1, -1] {
                xyz.sort_by_key(|&(xi, yi, zi)| f1 * xi + f2 * yi + f3 * zi);
                xyz.reverse();
                let mut s = 0;
                for i in 0..m {
                    let (xi, yi, zi) = xyz[i];
                    s += f1 * xi + f2 * yi + f3 * zi;
                }
                if s > result {
                    result = s;
                }
            }
        }
    }
    println!("{}", result);
}
