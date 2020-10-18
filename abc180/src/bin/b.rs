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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        x: [f64; n],
    }
    let mut d1 = 0.;
    let mut d2 = 0.;
    let mut d3 = 0.;
    for i in 0..n {
        d1 += x[i].abs();
        d2 += x[i] * x[i];
        if x[i].abs() > d3 {
            d3 = x[i].abs();
        }
    }
    d2 = d2.sqrt();
    println!("{}", d1);
    println!("{}", d2);
    println!("{}", d3);
}
