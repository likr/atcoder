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
        n: Chars,
    }
    let m = n.len();
    let digits = n
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    let mut div = vec![0; 3];
    for i in 0..m {
        div[digits[i] % 3] += 1;
    }
    let s = (div[1] + div[2] * 2) % 3;
    if s == 1 {
        if div[1] > 0 && m > 1 {
            println!("1");
        } else if div[2] > 1 && m > 2 {
            println!("2");
        } else {
            println!("-1")
        }
    } else if s == 2 {
        if div[2] > 0 && m > 1 {
            println!("1");
        } else if div[1] > 1 && m > 2 {
            println!("2");
        } else {
            println!("-1");
        }
    } else {
        println!("0");
    }
}
