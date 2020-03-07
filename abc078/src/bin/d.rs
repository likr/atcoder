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
        _z: isize,
        w: isize,
        a: [isize; n],
    }
    let mut result = (a[n - 1] - w).abs();
    if n > 1 {
        result = max(result, (a[n - 1] - a[n - 2]).abs());
    }
    println!("{}", result);
}
