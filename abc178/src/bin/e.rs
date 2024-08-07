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
        xy: [(isize, isize); n],
    }

    let x_max = xy.iter().map(|&(xi, yi)| xi + yi).max().unwrap();
    let x_min = xy.iter().map(|&(xi, yi)| xi + yi).min().unwrap();
    let y_max = xy.iter().map(|&(xi, yi)| xi - yi).max().unwrap();
    let y_min = xy.iter().map(|&(xi, yi)| xi - yi).min().unwrap();

    println!("{}", max(x_max - x_min, y_max - y_min));
}
