use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::Ext;

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
    }
    let mut f = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        f.push(i * i);
    }
    debug!(f);
    let mut ans = n;
    for &fi in f.iter() {
        for &fj in f.iter() {
            if fi == fj {
                continue;
            }
            let max_k = min(n / fi, n / fj);
            debug!(fi, fj, max_k, f.upper_bound(&max_k));
            ans += max_k - (f.upper_bound(&max_k) - 1);
        }
    }
    println!("{}", ans);
}
