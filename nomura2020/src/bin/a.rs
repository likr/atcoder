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
        h1: usize,
        m1: usize,
        h2: usize,
        m2: usize,
        k: usize,
    }
    let t1 = 60 * h1 + m1;
    let t2 = 60 * h2 + m2;
    println!("{}", t2 - k - t1);
}
