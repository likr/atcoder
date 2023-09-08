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
        q: usize,
    }
    let mut heap = BinaryHeap::new();
    let mut s = 0;
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    x: i64,
                }
                heap.push((Reverse(x - s), x, s));
            }
            2 => {
                input! {
                    x: i64,
                }
                s += x;
            }
            _ => {
                if let Some((_, x, t)) = heap.pop() {
                    println!("{}", x + s - t);
                }
            }
        }
    }
}
