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
        xa: f64,
        ya: f64,
        xb: f64,
        yb: f64,
        xc: f64,
        yc: f64,
    }
    println!(
        "{}",
        ((xb - xa) * (yc - ya) - (yb - ya) * (xc - xa)).abs() / 2.
    );
}
