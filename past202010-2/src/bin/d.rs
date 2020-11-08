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
        s: Chars,
    }
    let mut result = (INF, INF);
    for x in 0..=n {
        for y in 0..=n {
            let mut t = s.clone();
            for _ in 0..x {
                for i in 1..n {
                    if t[i] == '#' {
                        t[i - 1] = '#';
                    }
                }
            }
            for _ in 0..y {
                for i in (0..n - 1).rev() {
                    if t[i] == '#' {
                        t[i + 1] = '#';
                    }
                }
            }
            if t.iter().all(|&ti| ti == '#') {
                result = min(result, (x, y));
            }
        }
    }
    println!("{} {}", result.0, result.1);
}
