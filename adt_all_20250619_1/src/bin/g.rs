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
        mut s: Chars,
        q: usize,
        txc: [(usize, usize, char); q],
    }
    let mut last = None;
    for i in 0..q {
        let (ti, _, _) = txc[i];
        if ti != 1 {
            last = Some(i);
        }
    }
    for i in 0..q {
        let (ti, xi, ci) = txc[i];
        if ti == 1 {
            s[xi - 1] = ci;
        } else if last == Some(i) {
            for j in 0..n {
                s[j] = if ti == 2 {
                    s[j].to_ascii_lowercase()
                } else {
                    s[j].to_ascii_uppercase()
                };
            }
        }
    }
    println!("{}", s.iter().collect::<String>());
}
