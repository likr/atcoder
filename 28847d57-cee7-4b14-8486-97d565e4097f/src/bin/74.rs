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
        q: usize,
        tx: [(usize, i64); q],
    }
    let mut a = vec![-1; 1 << 20];
    let mut indices = (0..1 << 20).collect::<BTreeSet<usize>>();
    for &(ti, xi) in tx.iter() {
        if ti == 1 {
            let k = if let Some(&k) = indices.range((xi as usize - 1) % a.len()..).nth(0) {
                k
            } else {
                *indices.range(0..).nth(0).unwrap()
            };
            a[k] = xi;
            indices.remove(&k);
        } else {
            println!("{}", a[(xi as usize - 1) % a.len()]);
        }
    }
}
