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
        a: [[Usize1]; m],
    }
    let mut result = 0;
    for x in 1..1 << m {
        let mut mask = vec![false; n];
        for i in 0..m {
            if 1 << i & x > 0 {
                for &aij in a[i].iter() {
                    mask[aij] = true;
                }
            }
        }
        if mask.iter().all(|&f| f) {
            result += 1;
        }
    }
    println!("{}", result);
}
