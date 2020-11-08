use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }
    a.sort();
    b.sort();
    c.sort();
    let mut result = 0;
    for i in 0..n {
        let bi = b[i];
        let x = a.lower_bound(&bi);
        let y = n - c.upper_bound(&bi);
        result += x * y;
    }
    println!("{}", result);
}
