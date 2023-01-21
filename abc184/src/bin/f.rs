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
        t: usize,
        a: [usize; n],
    }
    let m = min(n, 20);
    let mut states = vec![];
    for x in 0..1 << m {
        let mut s = 0;
        for i in 0..m {
            if 1 << i & x > 0 {
                s += a[i];
            }
        }
        if s <= t {
            states.push(s);
        }
    }
    states.sort();
    states.dedup();
    if m == n {
        println!("{}", states[states.len() - 1]);
        return;
    }

    let mut result = 0;
    let k = n - m;
    for x in 0..1 << k {
        let mut s = 0;
        for i in 0..k {
            if 1 << i & x > 0 {
                s += a[m + i];
            }
        }
        if s > t {
            continue;
        }
        let j = states.upper_bound(&(t - s));
        result = max(result, s + states[j - 1]);
    }
    println!("{}", result);
}
