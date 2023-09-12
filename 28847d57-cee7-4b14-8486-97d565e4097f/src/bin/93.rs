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
    let mut count = vec![vec![0; w + 1]; h + 1];
    for &(ai, bi) in ab.iter() {
        count[ai][bi] = 1;
    }
    for i in 0..h {
        for j in 0..w {
            count[i + 1][j + 1] += count[i][j + 1] + count[i + 1][j] - count[i][j];
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let mut ok = 0;
            let mut ng = min(h - i, w - j) + 1;
            while ng - ok > 1 {
                let size = (ng + ok) / 2;
                if count[i + size][j + size] + count[i][j] - count[i + size][j] - count[i][j + size]
                    == 0
                {
                    ok = size;
                } else {
                    ng = size;
                }
            }
            ans += ok;
        }
    }
    println!("{}", ans);
}
