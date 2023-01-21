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
        mut xy: [(i64, i64); n],
    }
    xy.sort_by_key(|&(xi, _)| xi);
    let xm = if n % 2 == 0 {
        (xy[n / 2 - 1].0 + xy[n / 2].0) / 2
    } else {
        xy[n / 2].0
    };
    xy.sort_by_key(|&(_, yi)| yi);
    let ym = if n % 2 == 0 {
        (xy[n / 2 - 1].1 + xy[n / 2].1) / 2
    } else {
        xy[n / 2].1
    };
    let mut result = 0;
    for i in 0..n {
        let (xi, yi) = xy[i];
        result += (xi - xm).abs() + (yi - ym).abs();
    }
    println!("{}", result);
}
