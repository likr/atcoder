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
        x: usize,
        y: usize,
    }
    if x == y {
        println!("{}", x);
        return;
    }
    let mut s = HashSet::new();
    s.insert(0);
    s.insert(1);
    s.insert(2);
    s.remove(&x);
    s.remove(&y);
    for &z in s.iter() {
        println!("{}", z);
    }
}
