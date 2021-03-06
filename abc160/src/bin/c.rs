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
        k: usize,
        n: usize,
        a: [usize; n],
    }
    let mut result = k - (k - a[n - 1] + a[0]);
    for i in 1..n {
        result = min(result, k - a[i] + a[i - 1]);
    }
    println!("{}", result);
}
