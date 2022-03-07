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
        n: u32,
        m: usize,
        sc: [(u32, u32); m],
    }
    if n == 1 && sc.iter().all(|&(s, c)| (0 / 10u32.pow(n - s)) % 10 == c) {
        println!("{}", 0);
        return;
    }
    for x in 10u32.pow(n - 1)..10u32.pow(n) {
        if sc.iter().all(|&(s, c)| (x / 10u32.pow(n - s)) % 10 == c) {
            println!("{}", x);
            return;
        }
    }
    println!("-1");
}
