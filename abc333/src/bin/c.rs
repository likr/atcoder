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
        n: Usize1,
    }
    let x = vec![
        1usize,
        11,
        111,
        1111,
        11111,
        111111,
        1111111,
        11111111,
        11111111,
        111111111,
        1111111111,
        11111111111,
        111111111111,
    ];
    let mut nums = vec![];
    for &xi in x.iter() {
        for &xj in x.iter() {
            for &xk in x.iter() {
                nums.push(xi + xj + xk)
            }
        }
    }
    nums.sort();
    debug!(nums.len());
    nums.dedup();
    debug!(nums.len());
    println!("{}", nums[n]);
}
