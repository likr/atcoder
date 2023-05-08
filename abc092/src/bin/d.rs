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
        a: usize,
        b: usize,
    }
    let h = 100;
    let w = 100;
    let mut s = vec![vec!['.'; w]; h];
    for i in 0..h / 2 {
        for j in 0..w {
            s[i][j] = '#';
        }
    }
    let mut i = 1;
    let mut j = 1;
    for _ in 0..a - 1 {
        s[i][j] = '.';
        j += 2;
        if j >= w {
            j = 1;
            i += 2;
        }
    }
    let mut i = h / 2 + 1;
    let mut j = 1;
    for _ in 0..b - 1 {
        s[i][j] = '#';
        j += 2;
        if j >= w {
            j = 1;
            i += 2;
        }
    }
    println!("{} {}", h, w);
    for i in 0..h {
        println!(
            "{}",
            s[i].iter()
                .map(|&c| format!("{}", c))
                .collect::<Vec<_>>()
                .join("")
        );
    }
}
