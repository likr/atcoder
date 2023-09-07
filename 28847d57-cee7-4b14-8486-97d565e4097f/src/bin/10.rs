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
        l: usize,
    }
    let mut nums = (1..12).collect::<HashSet<_>>();
    let mut s = 1;
    for i in 1..12 {
        let mut k = l - 12 + i;
        for j in 1..12 {
            if nums.contains(&j) && k % j == 0 {
                k /= j;
                nums.remove(&j);
            }
        }
        s *= k;
    }
    for &v in nums.iter() {
        s /= v;
    }
    println!("{}", s);
}
