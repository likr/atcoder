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
        ab: [(usize, usize); n],
    }
    let mut k = 0;
    for i in 1..n {
        if ab[i].1 < ab[k].1 {
            k = i;
        }
    }
    println!("{}", ab[k].0 + ab[k].1);
}
