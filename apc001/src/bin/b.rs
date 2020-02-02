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
      a: [u64; n],
      b: [u64; n],
    }
    let mut x = 0;
    let mut y = 0;
    for i in 0..n {
        if a[i] > b[i] {
            x += a[i] - b[i];
        } else {
            y += (b[i] - a[i]) / 2;
        }
    }
    if y >= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
