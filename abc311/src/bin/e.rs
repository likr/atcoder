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
        h: usize,
        w: usize,
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut acc = vec![vec![0; w + 1]; h + 1];
    for k in 0..n {
        let (ai, bi) = ab[k];
        acc[ai][bi] = 1;
    }
    for i in 0..h {
        for j in 0..w {
            acc[i + 1][j + 1] += acc[i + 1][j] + acc[i][j + 1] - acc[i][j];
        }
    }
    let mut ans = 0usize;
    for i in 0..h {
        for j in 0..w {
            let mut ok = 0;
            let mut ng = min(h, w) + 1;
            while ng - ok > 1 {
                let k = (ok + ng) / 2;
                if i + k <= h
                    && j + k <= w
                    && acc[i + k][j + k] + acc[i][j] - acc[i][j + k] - acc[i + k][j] == 0
                {
                    ok = k;
                } else {
                    ng = k;
                }
            }
            debug!(i, j, ok);
            ans += ok;
        }
    }
    println!("{}", ans);
}
