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
        a: Chars,
        b: Chars,
    }
    let mut a = a
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .rev()
        .collect::<Vec<_>>();
    let mut b = b
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .rev()
        .collect::<Vec<_>>();
    while a.len() < b.len() {
        a.push(0);
    }
    while b.len() < a.len() {
        b.push(0);
    }
    let n = a.len();
    for i in 0..n {
        if a[i] + b[i] > 9 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
