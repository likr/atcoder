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
    let p = vec![
        (0, 0, '#'),
        (0, 1, '#'),
        (0, 2, '#'),
        (0, 3, '.'),
        (1, 0, '#'),
        (1, 1, '#'),
        (1, 2, '#'),
        (1, 3, '.'),
        (2, 0, '#'),
        (2, 1, '#'),
        (2, 2, '#'),
        (2, 3, '.'),
        (3, 0, '.'),
        (3, 1, '.'),
        (3, 2, '.'),
        (3, 3, '.'),
        (5, 5, '.'),
        (5, 6, '.'),
        (5, 7, '.'),
        (5, 8, '.'),
        (6, 5, '.'),
        (6, 6, '#'),
        (6, 7, '#'),
        (6, 8, '#'),
        (7, 5, '.'),
        (7, 6, '#'),
        (7, 7, '#'),
        (7, 8, '#'),
        (8, 5, '.'),
        (8, 6, '#'),
        (8, 7, '#'),
        (8, 8, '#'),
    ];
    let mut ans = vec![];
    for i in 0..n - 8 {
        for j in 0..m - 8 {
            if p.iter().all(|&(di, dj, c)| s[i + di][j + dj] == c) {
                ans.push((i + 1, j + 1));
            }
        }
    }
    for &(i, j) in ans.iter() {
        println!("{} {}", i, j);
    }
}
