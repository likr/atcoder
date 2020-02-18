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
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }
    if (a - c).abs() <= d || ((a - b).abs() <= d && (b - c).abs() <= d) {
        println!("Yes");
    } else {
        println!("No");
    }
}
