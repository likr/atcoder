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
        a: [usize; n],
    }
    let mut result = INF;
    for x in 0..2usize.pow((n - 1) as u32) {
        let mut vals = vec![];
        let mut s = a[0];
        for i in 1..n {
            if 1 << (i - 1) & x > 0 {
                vals.push(s);
                s = 0;
            }
            s |= a[i];
        }
        vals.push(s);
        let mut current = 0;
        for &v in vals.iter() {
            current ^= v;
        }
        result = min(result, current);
    }
    println!("{}", result);
}
