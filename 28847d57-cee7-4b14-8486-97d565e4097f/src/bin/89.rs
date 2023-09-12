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
        m: usize,
        mut h: [i64; n],
        w: [i64; m],
    }
    h.sort();
    let mut l = vec![0];
    for i in (1..n).step_by(2) {
        l.push(h[i] - h[i - 1]);
    }
    let mut r = vec![0];
    for i in (1..n).rev().step_by(2) {
        debug!(i + 1, i);
        r.push(h[i] - h[i - 1]);
    }
    r.reverse();
    for i in 1..l.len() {
        l[i] += l[i - 1];
    }
    for i in (1..r.len()).rev() {
        r[i - 1] += r[i];
    }
    let mut ans = INF as i64;
    for i in 0..m {
        let k = h.lower_bound(&w[i]);
        if k % 2 == 0 {
            ans = min(ans, h[k] - w[i] + l[k / 2] + r[k / 2]);
        } else {
            ans = min(ans, w[i] - h[k - 1] + l[k / 2] + r[k / 2]);
        }
    }
    println!("{}", ans);
}
