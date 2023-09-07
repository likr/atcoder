use itertools::*;
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
        x: usize,
        a: [[usize]; n],
    }
    let mut ans = 0;
    'outer: for items in a.iter().multi_cartesian_product() {
        let mut s = 1usize;
        for &&v in items.iter() {
            if let Some(t) = s.checked_mul(v) {
                s = t;
            } else {
                continue 'outer;
            }
        }
        if s == x {
            ans += 1;
        }
    }
    println!("{}", ans);
}
