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
        a: [usize; n],
    }
    let mut ans = INF;
    for x in 0..1 << (n - 1) {
        let mut values = vec![];
        values.push(a[0]);
        for i in 1..n {
            if x & 1 << (i - 1) > 0 {
                let m = values.len();
                values[m - 1] |= a[i];
            } else {
                values.push(a[i]);
            }
        }
        let mut v = values[0];
        for j in 1..values.len() {
            v ^= values[j];
        }
        ans = min(ans, v);
    }
    println!("{}", ans);
}
