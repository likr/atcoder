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
        a: usize,
        b: usize,
        d: [usize; n],
    }
    let c = a + b;
    let mut x = vec![];
    for i in 0..n {
        x.push(d[i] % c);
    }
    x.sort();
    x.dedup();
    let m = x.len();
    if x[0] + c - x[m - 1] - 1 >= b {
        println!("Yes");
        return;
    }
    for i in 1..m {
        if x[i] - x[i - 1] - 1 >= b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
