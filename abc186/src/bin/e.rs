use ac_library::*;
use num_integer::*;
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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: i64,
            s: i64,
            k: i64,
        }
        let d = k.gcd(&(n - s)).gcd(&n);
        if k.gcd(&n) == d {
            println!("{}", inv_mod(k / d, n / d) * (n - s) / d % (n / d));
        } else {
            println!("-1");
        }
    }
}
