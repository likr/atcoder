use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::f64;
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
    let cx = (x1 + x2) / 2.;
    let cy = (y1 + y2) / 2.;
    let t = 2. * f64::consts::PI / n as f64;
    let x0 = x1 - cx;
    let y0 = y1 - cy;
    let x = x0 * t.cos() - y0 * t.sin();
    let y = x0 * t.sin() + y0 * t.cos();
    println!("{} {}", x + cx, y + cy);
}
