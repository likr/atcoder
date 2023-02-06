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
        p: [(i64, i64); 4],
    }
    let dx1 = p[0].0 - p[2].0;
    let dy1 = p[0].1 - p[2].1;
    let dx2 = p[1].0 - p[3].0;
    let dy2 = p[1].1 - p[3].1;
    let f1 = dx2 * p[0].1 >= dy2 * (p[0].0 - p[1].0) + dx2 * p[1].1;
    let f2 = dx1 * p[1].1 >= dy1 * (p[1].0 - p[0].0) + dx1 * p[0].1;
    let f3 = dx2 * p[2].1 >= dy2 * (p[2].0 - p[1].0) + dx2 * p[1].1;
    let f4 = dx1 * p[3].1 >= dy1 * (p[3].0 - p[0].0) + dx1 * p[0].1;
    if f1 != f3 && f2 != f4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
