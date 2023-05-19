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
        a: [Usize1; n],
    }
    let k = 20;
    let mut d = vec![vec![0; n]; k];
    for j in 0..n {
        d[0][j] = a[j];
    }
    for i in 1..k {
        for j in 0..n {
            d[i][j] = d[i - 1][d[i - 1][j]];
        }
    }
    println!("{}", d[k - 1].iter().collect::<HashSet<_>>().len());
}
