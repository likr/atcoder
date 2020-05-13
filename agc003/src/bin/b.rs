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
    let mut s = 0usize;
    for i in 0..n {
        if a[i] == 0 {
            result += s / 2;
            s = 0;
        } else {
            s += a[i];
        }
    }
    result += s / 2;
    println!("{}", result);
}
