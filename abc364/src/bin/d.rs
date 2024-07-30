use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        q: usize,
        mut a: [i64; n],
        bk: [(i64, usize); q],
    }
    a.sort();
    for &(bi, ki) in bk.iter() {
        let j = a.lower_bound(&bi);
        let mut ok = INF as i64;
        let mut ng = -1;
        while ok - ng > 1 {
            let d = (ok + ng) / 2;
            let m1 = j - a[..j].lower_bound_by_key(&(-d), |&ai| ai - bi);
            let m2 = a[j..].upper_bound_by_key(&d, |&ai| ai - bi);
            if m1 + m2 >= ki {
                ok = d;
            } else {
                ng = d;
            }
        }
        println!("{}", ok);
    }
}
