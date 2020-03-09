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
        a: [usize; n],
    }
    let mut b = vec![0usize; 100000];
    for &ai in &a {
        b[ai] += 1;
    }
    let mut result = 0;
    for i in 2..b.len() {
        result = max(result, b[i - 2] + b[i - 1] + b[i]);
    }
    println!("{}", result);
}
