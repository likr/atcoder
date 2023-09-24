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
        x: i64,
        mut a: [i64; n - 1],
    }
    a.sort();
    a.reverse();
    let s = a.iter().sum::<i64>() - a[0] - a[n - 2];
    if x - s <= a[0] {
        if x - s <= a[n - 2] {
            println!("0");
        } else {
            println!("{}", x - s);
        }
    } else {
        println!("-1");
    }
}
