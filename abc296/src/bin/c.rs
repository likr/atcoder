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
        mut x: i64,
        mut a: [i64; n],
    }
    a.sort();
    x = x.abs();

    let mut j = 0;
    for i in 0..n {
        while j < n && a[j] - a[i] < x {
            j += 1;
        }
        if j == n {
            break;
        }
        if a[j] - a[i] == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
