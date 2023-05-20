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
        n: u32,
        m: usize,
        sc: [(u32, usize); m],
    }
    'outer: for x in 0..10usize.pow(n) {
        for i in 0..m {
            let (si, ci) = sc[i];
            if x / 10usize.pow(n - si) % 10 != ci {
                continue 'outer;
            }
        }
        if n == 1 || x >= 10usize.pow(n - 1) {
            println!("{}", x);
            return;
        }
    }
    println!("-1");
}
