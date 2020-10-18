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
        s: [usize; 4],
    }
    for x in 0..16 {
        let mut a = 0;
        let mut b = 0;
        for i in 0..4 {
            if x & 1 << i > 0 {
                a += s[i];
            } else {
                b += s[i];
            }
            if a == b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
