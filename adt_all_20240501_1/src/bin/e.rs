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
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m],
    }
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();
    let mut ans = INF as i64;
    while !a.is_empty() && !b.is_empty() {
        ans = min(ans, (a.last().unwrap() - b.last().unwrap()).abs());
        if a.last().unwrap() < b.last().unwrap() {
            a.pop();
        } else {
            b.pop();
        }
    }
    println!("{}", ans);
}
