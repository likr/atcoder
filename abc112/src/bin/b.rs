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
        t: usize,
        ct: [(usize, usize); n],
    }
    if let Some(result) = ct.iter().filter(|(_, ti)| *ti <= t).map(|(ci, _)| ci).min() {
        println!("{}", result);
    } else {
        println!("TLE");
    }
}
