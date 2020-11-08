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
        m: usize,
        s: [Chars; n],
    }
    let mut t = vec![vec!['.'; m + 2]; n + 2];
    for i in 0..n {
        for j in 0..m {
            t[i + 1][j + 1] = s[i][j];
        }
    }

    for i in 0..n {
        for j in 0..m {
            let mut count = 0;
            for di in 0..3 {
                for dj in 0..3 {
                    if t[i + di][j + dj] == '#' {
                        count += 1;
                    }
                }
            }
            print!("{}", count);
        }
        println!("");
    }
}
