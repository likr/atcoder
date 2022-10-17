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
        p: [usize; n],
    }
    let mut nums = BTreeSet::new();
    for i in 0..k {
        nums.insert(p[i]);
    }
    println!("{}", nums.range(0..).nth(0).unwrap());
    for i in k..n {
        nums.insert(p[i]);
        let v = *nums.range(0..).nth(0).unwrap();
        nums.remove(&v);
        println!("{}", nums.range(0..).nth(0).unwrap());
    }
}
