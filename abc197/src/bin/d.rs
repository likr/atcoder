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
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    }
    let xm = (x1 + x2) / 2.;
    let ym = (y1 + y2) / 2.;
    let r = ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt() / 2.;
    let dt = 2. * PI / n as f64;
    let t = (y1 - ym).atan2(x1 - xm);
    println!("{} {}", r * (t + dt).cos() + xm, r * (t + dt).sin() + ym);
}
