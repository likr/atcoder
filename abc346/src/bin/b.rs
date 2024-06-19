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
    let t = "wbwbwwbwbwbw".chars().collect::<Vec<_>>();
    let mut s = vec![];
    for _ in 0..30 {
        for &c in t.iter() {
            s.push(c);
        }
    }
    let n = s.len();
    let mut w_acc = vec![0; n + 1];
    let mut b_acc = vec![0; n + 1];
    for i in 0..n {
        if s[i] == 'w' {
            w_acc[i + 1] = 1;
        } else {
            b_acc[i + 1] = 1;
        }
    }
    for i in 0..n {
        w_acc[i + 1] += w_acc[i];
        b_acc[i + 1] += b_acc[i];
    }
    for j in 1..=n {
        for i in 0..j {
            if w_acc[j] - w_acc[i] == w && b_acc[j] - b_acc[i] == b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
