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
        w: usize,
        b: usize,
    }
    let s = "wbwbwwbwbwbw".repeat(100).chars().collect::<Vec<_>>();
    let n = s.len();
    let mut w_count = vec![0; n + 1];
    let mut b_count = vec![0; n + 1];
    for i in 0..n {
        if s[i] == 'w' {
            w_count[i + 1] = 1;
        } else {
            b_count[i + 1] = 1;
        }
    }
    for i in 0..n {
        w_count[i + 1] += w_count[i];
        b_count[i + 1] += b_count[i];
    }
    for j in 0..=n {
        for i in 0..j {
            if w_count[j] - w_count[i] == w && b_count[j] - b_count[i] == b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
