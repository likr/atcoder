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
        a: [isize; n],
    }
    let mut result = 0;
    for i in 0..n {
        for j in 0..i {
            result = max(result, (a[i] - a[j]).abs());
        }
    }
    println!("{}", result);
}
