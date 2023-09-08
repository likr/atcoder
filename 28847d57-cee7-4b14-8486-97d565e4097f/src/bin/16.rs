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
    let mut left = vec![];
    let mut right = vec![];
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        match t {
            1 => {
                left.push(x);
            }
            2 => {
                right.push(x);
            }
            _ => {
                let x = x - 1;
                if x < left.len() {
                    println!("{}", left[left.len() - 1 - x]);
                } else {
                    println!("{}", right[x - left.len()])
                }
            }
        }
    }
}
