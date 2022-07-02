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
        mut a: [usize; m],
    }
    a.push(0);
    a.push(n + 1);
    a.sort();
    let m = a.len();
    let mut k = INF;
    for i in 1..m {
        let d = a[i] - a[i - 1] - 1;
        if d > 0 {
            k = min(k, d);
        }
    }
    if k == INF {
        println!("0");
        return;
    }
    let mut result = 0;
    for i in 1..m {
        let d = a[i] - a[i - 1] - 1;
        result += (d + k - 1) / k;
    }
    println!("{}", result);
}
