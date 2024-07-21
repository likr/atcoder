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
        mut n: Usize1,
    }
    if n == 0 {
        println!("0");
        return;
    }
    let mut b = 9;
    let mut c = 1;
    let mut d = 1;
    while n > b {
        n -= b;
        d += 1;
        if d % 2 == 1 {
            c *= 10;
            b *= 10;
        }
    }
    debug!(n, b, d);
    let mut s = vec![];
    for c in format!("{}", n + c - 1).chars() {
        s.push(c);
    }
    if d % 2 == 1 {
        s.pop();
    }
    for c in format!("{}", n + c - 1).chars().rev() {
        s.push(c);
    }
    println!("{}", s.iter().collect::<String>());
}
