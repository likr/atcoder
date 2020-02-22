use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
    }
    let mut l = vec![0u64; n + 1];
    l[0] = 2;
    l[1] = 1;
    for i in 2..=n {
        l[i] = l[i - 1] + l[i - 2];
    }
    println!("{}", l[n]);
}
