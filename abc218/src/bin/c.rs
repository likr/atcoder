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

fn trim(s: Vec<Vec<char>>, w: usize, h: usize) -> Vec<Vec<char>> {
    let left = (0..w)
        .filter(|&j| (0..h).any(|i| s[i][j] == '#'))
        .min()
        .unwrap();
    let right = (0..w)
        .filter(|&j| (0..h).any(|i| s[i][j] == '#'))
        .max()
        .unwrap();
    let top = (0..h)
        .filter(|&i| (0..w).any(|j| s[i][j] == '#'))
        .min()
        .unwrap();
    let bottom = (0..h)
        .filter(|&i| (0..w).any(|j| s[i][j] == '#'))
        .max()
        .unwrap();
    let w2 = right - left + 1;
    let h2 = bottom - top + 1;
    let mut t = vec![vec![' '; w2]; h2];
    for i in 0..h2 {
        for j in 0..w2 {
            t[i][j] = s[i + top][j + left];
        }
    }
    t
}

fn rotate(s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = s.len();
    let w = s[0].len();
    let mut t = vec![vec![' '; h]; w];
    for i in 0..h {
        for j in 0..w {
            t[j][h - 1 - i] = s[i][j];
        }
    }
    t
}

fn main() {
    input! {
        n: usize,
        s: [Chars;n ],
        t: [Chars; n],
    }
    let mut trim_s = trim(s, n, n);
    let trim_t = trim(t, n, n);
    for _ in 0..4 {
        if trim_s == trim_t {
            println!("Yes");
            return;
        }
        trim_s = rotate(trim_s);
    }
    println!("No");
}
