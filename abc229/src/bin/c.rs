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
        mut w: usize,
        mut ab: [(usize, usize); n],
    }
    ab.sort_by_key(|&(ai, _)| ai);
    ab.reverse();
    let mut result = 0usize;
    for i in 0..n {
        if ab[i].1 <= w {
            result += ab[i].0 * ab[i].1;
            w -= ab[i].1;
        } else {
            result += ab[i].0 * w;
            break;
        }
    }
    println!("{}", result);
}
