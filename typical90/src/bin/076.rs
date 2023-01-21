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
        mut a: [i64; n],
    }
    let mut s = 0;
    for i in 0..n {
        s += a[i];
        a.push(a[i]);
    }
    let mut t = 0;
    let mut j = 0;
    let mut ok = false;
    for i in 0..n {
        while 10 * (t + a[j]) <= s {
            t += a[j];
            j += 1;
        }
        if 10 * t == s {
            ok = true;
        }
        t -= a[i];
    }
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
