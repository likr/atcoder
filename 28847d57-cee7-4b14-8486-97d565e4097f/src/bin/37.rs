use num::integer::gcd;
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
        xy: [(i64, i64); n],
    }
    let mut ans = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let g = gcd((xy[i].0 - xy[j].0).abs(), (xy[i].1 - xy[j].1).abs());
            ans.insert(((xy[i].0 - xy[j].0) / g, (xy[i].1 - xy[j].1) / g));
        }
    }
    println!("{}", ans.len());
}
