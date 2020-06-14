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
        w: f64,
        h: f64,
        x: f64,
        y: f64,
    }
    if 2. * x == w && 2. * y == h {
        println!("{} 1", w * h / 2.0);
    } else {
        println!("{} 0", w * h / 2.0);
    }
}
