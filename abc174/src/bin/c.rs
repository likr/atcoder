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
        k: usize,
    }
    let mut visited = HashSet::new();
    let mut b = 0;
    for i in 1.. {
        b = (b * 10 + 7) % k;
        if visited.contains(&(b % k)) {
            println!("-1");
            return;
        }
        visited.insert(b % k);
        if b % k == 0 {
            println!("{}", i);
            return;
        }
    }
}
