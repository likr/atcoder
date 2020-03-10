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
        k: usize,
        mut r: [usize; n],
    }
    r.sort();
    let mut s = 0.;
    for i in n - k..n {
        s = (s + r[i] as f64) / 2.;
    }
    println!("{}", s);
}
