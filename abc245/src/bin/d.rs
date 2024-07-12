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
        mut a: [i64; n + 1],
        mut c: [i64; n + m + 1],
    }
    a.reverse();
    c.reverse();
    let mut b = vec![];
    for j in 0..=m {
        let bj = c[j] / a[0];
        for i in 0..=n {
            c[i + j] -= a[i] * bj;
        }
        b.push(format!("{}", bj));
    }
    b.reverse();
    println!("{}", b.join(" "));
}
