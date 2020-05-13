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
        mut a: [usize; n * 3],
    }
    a.sort();
    a.reverse();
    let mut s = 0usize;
    for i in 0..n {
        s += a[2 * i + 1];
    }
    println!("{}", s);
}
