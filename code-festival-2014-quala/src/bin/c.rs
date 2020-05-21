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
    }
    let a = a - 1;
    let mut result = 0usize;
    result += b / 4;
    result += a / 100;
    result += b / 400;
    result -= a / 4;
    result -= b / 100;
    result -= a / 400;
    println!("{}", result);
}
