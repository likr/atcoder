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
        x: [usize; n],
    }
    let mut y = x.clone();
    y.sort();
    for i in 0..n {
        if x[i] >= y[n / 2] {
            println!("{}", y[n / 2 - 1]);
        } else {
            println!("{}", y[n / 2]);
        }
    }
}
