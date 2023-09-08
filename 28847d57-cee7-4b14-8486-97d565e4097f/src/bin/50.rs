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
        q: usize,
        mut a: [usize; n],
        txy: [(usize, usize, usize); q],
    }
    let mut shift = 0;
    for &(ti, xi, yi) in txy.iter() {
        match ti {
            1 => {
                a.swap(
                    (xi - 1 + (n - shift % n)) % n,
                    (yi - 1 + (n - shift % n)) % n,
                );
            }
            2 => {
                shift += 1;
            }
            _ => {
                println!("{}", a[(xi - 1 + (n - shift % n)) % n]);
            }
        }
    }
}
