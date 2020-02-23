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
    let mut s = 0;
    let mut k = 0;
    while s < n {
        k += 1;
        s += k;
    }
    for k in 1..=k {
        if k != s - n {
            println!("{}", k);
        }
    }
}
