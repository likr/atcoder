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

fn cost(x: usize, a: usize, b: usize) -> usize {
    let mut y = x;
    let mut d = 0;
    while y > 0 {
        y /= 10;
        d += 1;
    }
    a * x + b * d
}

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize
    }
    if x < cost(1, a, b) {
        println!("0");
        return;
    }
    let mut ok = 1;
    let mut ng = 1000000001;
    while ng - ok > 1 {
        let p = (ok + ng) / 2;
        if cost(p, a, b) <= x {
            ok = p;
        } else {
            ng = p;
        }
    }
    println!("{}", ok);
}
