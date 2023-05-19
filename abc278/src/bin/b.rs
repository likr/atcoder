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
        mut h: usize,
        mut m: usize,
    }

    loop {
        let a = h / 10;
        let b = h % 10;
        let c = m / 10;
        let d = m % 10;
        let h2 = a * 10 + c;
        let m2 = b * 10 + d;
        if h2 <= 23 && m2 <= 59 {
            println!("{} {}", h, m);
            return;
        }
        if m == 59 {
            h += 1;
            m = 0;
            if h == 24 {
                h = 0;
            }
        } else {
            m += 1;
        }
    }
}
