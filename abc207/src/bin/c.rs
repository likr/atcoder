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
    }
    let mut lr = vec![];
    for _ in 0..n {
        input! {
            ti: usize,
            li: usize,
            ri: usize,
        }
        match ti {
            1 => {
                lr.push((2 * li, 2 * ri + 1));
            }
            2 => {
                lr.push((2 * li, 2 * ri));
            }
            3 => {
                lr.push((2 * li + 1, 2 * ri + 1));
            }
            _ => {
                lr.push((2 * li + 1, 2 * ri));
            }
        }
    }
    let m = lr.len();
    let mut result = 0;
    for i in 0..m {
        for j in 0..i {
            let (li, ri) = lr[i];
            let (lj, rj) = lr[j];
            if (li < rj && lj < ri) || (lj < ri && li < rj) {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
