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
        mut a: usize,
        mut b: usize,
        k: usize,
    }
    for i in 0..k {
        if i % 2 == 0 {
            b += a / 2;
            a /= 2;
        } else {
            a += b / 2;
            b /= 2;
        }
    }
    println!("{} {}", a, b);
}
