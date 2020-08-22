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
        mut a: [usize; n],
    }
    let mut result = 0usize;
    for i in 1..n {
        if a[i - 1] > a[i] {
            let b = a[i - 1] - a[i];
            result += b;
            a[i] += b;
        }
    }
    println!("{}", result);
}
