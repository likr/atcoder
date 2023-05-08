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
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut ok = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 1..w {
            if s[i][j] == '#' && s[i][j - 1] == '#' {
                ok[i][j] = true;
                ok[i][j - 1] = true;
            }
        }
    }
    for i in 1..h {
        for j in 0..w {
            if s[i][j] == '#' && s[i - 1][j] == '#' {
                ok[i][j] = true;
                ok[i - 1][j] = true;
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' && !ok[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
