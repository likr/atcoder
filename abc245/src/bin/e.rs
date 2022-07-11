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
        m: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; m],
        d: [i64; m],
    }
    let mut x = (0..n).collect::<Vec<_>>();
    x.sort_by(|&i, &j| {
        cmp( a[i] * b[j] , a[j] * b[i]) 
    })
    let mut y = (0..m).collect::<Vec<_>>();
    println!("{}", n);
}
