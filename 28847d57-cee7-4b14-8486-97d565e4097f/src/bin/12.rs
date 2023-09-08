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
    let mut count = vec![0; n + 1];
    for a in 1.. {
        if a * a > n {
            break;
        }
        for b in 1.. {
            if a * a + b * b + a * b > n {
                break;
            }
            for c in 1.. {
                let s = a * a + b * b + c * c + a * b + a * c + b * c;
                if s > n {
                    break;
                }
                count[s] += 1;
            }
        }
    }
    for i in 1..=n {
        println!("{}", count[i]);
    }
}
