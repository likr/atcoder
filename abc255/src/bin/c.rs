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
        x: i64,
        mut a: i64,
        mut d: i64,
        n: i64,
    }
    if d < 0 {
        a = a + d * (n - 1);
        d = -d;
    }
    if x > a + d * (n - 1) {
        println!("{}", x - (a + d * (n - 1)));
        return;
    }
    if x <= a {
        println!("{}", a - x);
        return;
    }
    let mut ok = n;
    let mut ng = 0;
    while ok - ng > 1 {
        let m = (ok + ng) / 2;
        let y = a + d * (m - 1);
        if y > x {
            ok = m;
        } else {
            ng = m;
        }
    }
    debug!(ok, ng);
    let y1 = a + d * (ok - 1);
    let y0 = a + d * (ng - 1);
    println!("{}", min(y1 - x, x - y0));
}
