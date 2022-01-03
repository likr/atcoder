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
        k: usize,
    }
    let m = 100000;
    let mut next = vec![INF; m];
    for i in 0..m {
        let mut x = i;
        let mut s = 0;
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        next[i] = (i + s) % m;
    }

    let mut current = n;
    let mut cycle = 1;
    while next[current] != n {
        cycle += 1;
        current = next[current];
    }
    let mut current = n;
    for _ in 0..k % cycle {
        current = next[current];
    }
    println!("{}", current);
}
