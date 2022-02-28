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
        s: [Chars; n],
    }
    for i in 5..n {
        for j in 0..n {
            let mut count = 0;
            for k in 0..6 {
                if s[i - k][j] == '#' {
                    count += 1;
                }
            }
            if count >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    for i in 0..n {
        for j in 5..n {
            let mut count = 0;
            for k in 0..6 {
                if s[i][j - k] == '#' {
                    count += 1;
                }
            }
            if count >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    for i in 5..n {
        for j in 5..n {
            let mut count = 0;
            for k in 0..6 {
                if s[i - k][j - k] == '#' {
                    count += 1;
                }
            }
            if count >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    for i in 5..n {
        for j in 0..n - 5 {
            let mut count = 0;
            for k in 0..6 {
                if s[i - k][j + k] == '#' {
                    count += 1;
                }
            }
            if count >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
