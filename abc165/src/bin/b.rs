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
        x: usize,
    }
    let mut y = 0;
    let mut z = 100;
    while z < x {
        z = (z as f64 * 1.01) as usize;
        y += 1;
    }
    println!("{}", y);
}
