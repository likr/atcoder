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
        n: usize,
        k: usize,
    }
    let mut s = 0f64;
    for i in 1..=n {
        let mut b = 1;
        let mut c = 0;
        while i * b < k {
            b *= 2;
            c += 1;
        }
        s += 1. / n as f64 / 2f64.powi(c as i32);
    }
    println!("{}", s);
}
