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
        x: Chars,
    }
    let x = x
        .split(|&c| c == '.')
        .map(|s| s.iter().collect::<String>().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    if x[1] / 100 < 5 {
        println!("{}", x[0]);
    } else {
        println!("{}", x[0] + 1);
    }
}
