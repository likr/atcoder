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
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    debug!(acc[n]);
    acc.push(INF);
    for i in 0..n {
        let k1 = acc.lower_bound(&(acc[i] + p));
        if acc[k1] != acc[i] + p {
            continue;
        }
        let k2 = acc.lower_bound(&(acc[i] + p + q));
        if acc[k2] != acc[i] + p + q {
            continue;
        }
        let k3 = acc.lower_bound(&(acc[i] + p + q + r));
        if acc[k3] != acc[i] + p + q + r {
            continue;
        }
        debug!(k1, k2, k3);
        println!("Yes");
        return;
    }
    println!("No");
}
