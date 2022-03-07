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
        mut a: [i64; n],
    }
    let mut left = BTreeMap::new();
    let mut right = BTreeMap::new();
    for i in 0..n {
        if let Some(&c) = right.get(&(i as i64 - a[i])) {
            right.insert(i as i64 - a[i], c + 1);
        } else {
            right.insert(i as i64 - a[i], 1);
        }
    }
    let mut result = 0usize;
    for i in 0..n {
        let &c = right.get(&(i as i64 - a[i])).unwrap();
        if c > 1 {
            right.insert(i as i64 - a[i], c - 1);
        } else {
            right.remove(&(i as i64 - a[i]));
        }
        if let Some(&c) = left.get(&(i as i64 - a[i])) {
            result += c;
        }
        if let Some(&c) = right.get(&(a[i] + i as i64)) {
            result += c;
        }
        if let Some(&c) = left.get(&(a[i] + i as i64)) {
            left.insert(a[i] + i as i64, c + 1);
        } else {
            left.insert(a[i] + i as i64, 1);
        }
    }
    println!("{}", result / 2);
}
