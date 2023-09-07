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
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q],
    }
    let r = l / 2.;
    for &ei in e.iter() {
        let theta = -FRAC_PI_2 - PI * 2. * ei / t;
        let y1 = r * theta.cos();
        let z = r * theta.sin() + r;
        let d = x.hypot(y1 - y);
        println!("{}", z.atan2(d).to_degrees());
    }
}
