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
        a: [usize; n],
        b: [usize; m],
    }
    let mut c = vec![];
    for i in 0..n {
        c.push((a[i], 0));
    }
    for i in 0..m {
        c.push((b[i], 1));
    }
    c.sort();
    for i in 1..c.len() {
        if c[i - 1].1 == 0 && c[i].1 == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
