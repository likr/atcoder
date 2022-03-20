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
        b: [usize; n],
    }
    let mut x = 0;
    let mut nums = HashSet::new();
    for i in 0..n {
        if a[i] == b[i] {
            x += 1;
        }
        nums.insert(a[i]);
    }
    let mut y = 0;
    for i in 0..n {
        if nums.contains(&b[i]) {
            y += 1;
        }
    }
    println!("{}", x);
    println!("{}", y - x);
}
