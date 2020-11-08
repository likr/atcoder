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
    }
    let mut d = vec![];
    let mut x = n;
    while x > 0 {
        d.push(x % 10);
        x /= 10;
    }
    let mut count = vec![0; 3];
    let k = d.len();
    for i in 0..k {
        count[d[i] % 3] += 1;
    }
    let s = d.iter().sum::<usize>();
    let result = match s % 3 {
        1 => {
            if count[1] >= 1 {
                Some(1)
            } else if count[2] >= 2 {
                Some(2)
            } else {
                None
            }
        }
        2 => {
            if count[2] >= 1 {
                Some(1)
            } else if count[1] >= 2 {
                Some(2)
            } else {
                None
            }
        }
        _ => Some(0),
    };
    if let Some(result) = result {
        if result < k {
            println!("{}", result);
        } else {
            println!("-1");
        }
    } else {
        println!("-1");
    }
}
