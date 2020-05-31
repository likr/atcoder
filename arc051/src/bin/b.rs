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
    }
    let mut f0;
    let mut f1 = 1usize;
    let mut f2 = 1usize;
    for _ in 0..k {
        f0 = f1;
        f1 = f2;
        f2 = f0 + f1;
    }
    println!("{} {}", f2, f1);
}
