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
        a: f64,
        b: f64,
        x: f64,
    }
    let mut ok = 0.;
    let mut ng = FRAC_PI_2;
    for _ in 0..30 {
        let t = (ok + ng) / 2.;
        if t > b.atan2(a) {
            if b / t.sin() * t.cos() * b / 2. * a <= x {
                ng = t;
            } else {
                ok = t;
            }
        } else {
            if a * a * b - a / (FRAC_PI_2 - t).sin() * (FRAC_PI_2 - t).cos() * a / 2. * a <= x {
                ng = t;
            } else {
                ok = t;
            }
        }
    }
    println!("{}", ok.to_degrees());
}
