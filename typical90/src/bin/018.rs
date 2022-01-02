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
        x0: f64,
        y0: f64,
        q: usize,
        e: [f64; q],
    }
    let r = l / 2.;
    for ei in e.iter() {
        let t = (360. * ei / t).to_radians();
        let x1 = 0.;
        let y1 = -r * t.sin();
        let z = -r * t.cos() + r;
        let dx = x1 - x0;
        let dy = y1 - y0;
        let d = (dx * dx + dy * dy).sqrt();
        println!("{}", z.atan2(d).to_degrees());
    }
}
