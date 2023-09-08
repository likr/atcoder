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
        mut x: usize,
        y: usize,
        a: usize,
        b: usize,
    }
    let mut ans = 0;
    loop {
        if let Some(v) = x.checked_mul(a) {
            if v < y && v < x + b {
                x = v;
            } else {
                break;
            }
        } else {
            break;
        }
        ans += 1;
    }
    ans += (y - x - 1) / b;
    println!("{}", ans);
}
