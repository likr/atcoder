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
        a: usize,
        b: usize,
        c: usize,
        k: usize,
        s: usize,
        t: usize,
    }
    if s + t >= k {
        println!("{}", (a - c) * s + (b - c) * t);
    } else {
        println!("{}", a * s + b * t);
    }
}
