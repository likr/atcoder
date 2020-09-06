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

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        hs: [(usize, usize); n],
    }
    let mut ng = 0;
    let mut ok = INF;
    while ok - ng > 1 {
        let m = (ok + ng) / 2;
        debug!(ng, m, ok);
        if (0..n).any(|i| hs[i].0 > m) {
            ng = m;
            continue;
        }
        let min_t = (0..n)
            .map(|i| {
                let (hi, si) = hs[i];
                (m - hi) / si
            })
            .collect::<Vec<usize>>();
        debug!(min_t);
        let mut count = vec![0; n];
        for i in 0..n {
            if min_t[i] >= n {
                count[n - 1] += 1;
            } else {
                count[min_t[i]] += 1;
            }
        }
        for i in 1..n {
            count[i] += count[i - 1];
        }
        debug!(count);
        if (0..n).any(|i| count[i] > i + 1) {
            ng = m;
        } else {
            ok = m;
        }
    }
    println!("{}", ok);
}
