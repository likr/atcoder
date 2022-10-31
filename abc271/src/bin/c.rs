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
        mut a: [usize; n],
    }
    a.sort();
    let mut l = 0;
    let mut u = INF;
    while u - l > 1 {
        let m = (u + l) / 2;
        let mut used = HashSet::new();
        let mut s = 0;
        for &ai in a.iter() {
            if ai <= m && !used.contains(&ai) {
                used.insert(ai);
            } else {
                s += 1;
            }
        }
        if s / 2 >= m - used.len() {
            l = m;
        } else {
            u = m;
        }
    }
    println!("{}", l);
}
