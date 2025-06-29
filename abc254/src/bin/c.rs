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
        k: usize,
        a: [usize; n],
    }
    let mut rows = vec![vec![]; k];
    for i in 0..n {
        rows[i % k].push(a[i]);
    }
    for i in 0..k {
        rows[i].sort();
    }
    debug!(rows);
    let mut b = vec![];
    for i in 0..n {
        b.push(rows[i % k][i / k]);
    }
    debug!(b);
    for i in 1..n {
        if b[i - 1] > b[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
