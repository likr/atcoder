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
        _n: usize,
        m: usize,
        x: usize,
        a: [usize; m],
    }

    let c1 = a.iter().filter(|&&ai| ai < x).count();
    let c2 = a.iter().filter(|&&ai| ai > x).count();
    println!("{}", min(c1, c2));
}
