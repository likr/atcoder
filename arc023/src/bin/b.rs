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
        r: usize,
        c: usize,
        d: usize,
        a: [[usize; c]; r],
    }
    let mut result = 0;
    for i in 0..r {
        for j in 0..c {
            if i + j <= d && (i + j) % 2 == d % 2 {
                result = max(result, a[i][j]);
            }
        }
    }
    println!("{}", result);
}
