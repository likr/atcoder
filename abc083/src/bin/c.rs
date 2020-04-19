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
        y: usize,
    }
    let mut count = 0;
    let mut z = x;
    while z <= y {
        count += 1;
        z *= x;
    }
    println!("{}", count);
}
