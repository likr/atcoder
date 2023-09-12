use ac_library::*;
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
        q: usize,
        a: [usize; n],
    }
    let mut bit = FenwickTree::new(n, 0);
    for i in 0..n {
        bit.add(i, a[i]);
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                p: usize,
                x: usize,
            }
            bit.add(p, x);
        } else {
            input! {
                l: usize,
                r: usize,
            }
            println!("{}", bit.sum(l..r));
        }
    }
}
