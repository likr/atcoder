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
        k: [usize; n - 1],
    }
    let mut l = vec![0; n];
    l[0] = k[0];
    l[n - 1] = k[n - 2];
    for i in 1..n - 1 {
        l[i] = min(k[i - 1], k[i]);
    }
    for i in 0..n {
        println!("{}", l[i]);
    }
}
