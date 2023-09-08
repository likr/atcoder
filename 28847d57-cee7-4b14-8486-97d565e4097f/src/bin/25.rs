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
    let mut ans = 0;
    for i in 0..n {
        let mut s = 0;
        for j in (0..i).rev() {
            if a[j] < a[i] {
                break;
            }
            s += a[i];
        }
        for j in i..n {
            if a[j] < a[i] {
                break;
            }
            s += a[i];
        }
        ans = max(ans, s);
    }
    println!("{}", ans);
}
