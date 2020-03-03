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
        mut a: [usize; 3],
        k: usize,
    }
    a.sort();
    for _ in 0..k {
        a[2] *= 2;
    }
    println!("{}", a.iter().sum::<usize>());
}
