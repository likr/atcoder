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
        q: usize,
        cx: [(usize, usize); q],
    }
    let mut intervals = BTreeMap::new();
    intervals.insert(0, l);
    for &(ci, xi) in cx.iter() {
        let (&k, &size) = intervals.range(..=xi).rev().nth(0).unwrap();
        if ci == 1 {
            intervals.remove(&k);
            let l1 = xi - k;
            intervals.insert(k, l1);
            let l2 = size - l1;
            intervals.insert(k + l1, l2);
        } else {
            println!("{}", size);
        }
    }
}
