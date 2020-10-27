use ordered_float::OrderedFloat;
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
        mut xy: [(f64, f64); n],
    }
    xy.sort_by_key(|&(xi, yi)| OrderedFloat::from(yi.atan2(xi) + PI));
    for i in 0..n {
        xy.push(xy[i]);
    }

    let mut result = 0.;
    for i in 0..n {
        for j in i..i + n {
            let mut x = 0.;
            let mut y = 0.;
            for k in i..=j {
                x += xy[k].0;
                y += xy[k].1;
            }
            let d = (x * x + y * y).sqrt();
            if d > result {
                result = d;
            }
        }
    }
    println!("{}", result);
}
