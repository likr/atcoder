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
        mut a: [i64; n],
    }
    a.sort();
    a.reverse();
    let d = ((a[0] + 1) / 2) as i64;
    let mut min_d = INF as i64;
    let mut b = 0;
    for i in 1..n {
        let d0 = (d - a[i]).abs();
        if d0 < min_d {
            min_d = d0;
            b = a[i];
        }
    }
    println!("{} {}", a[0], b);
}
