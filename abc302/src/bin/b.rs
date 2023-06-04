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
    for i in 4..h {
        for j in 0..w {
            if (0..5).map(|k| s[i - k][j]).collect::<Vec<_>>() == vec!['s', 'n', 'u', 'k', 'e'] {
                for k in 0..5 {
                    println!("{} {}", i - k + 1, j + 1);
                }
                return;
            }
            if (0..5).map(|k| s[i - k][j]).rev().collect::<Vec<_>>()
                == vec!['s', 'n', 'u', 'k', 'e']
            {
                for k in (0..5).rev() {
                    println!("{} {}", i - k + 1, j + 1);
                }
                return;
            }
        }
    }
    for i in 0..h {
        for j in 4..w {
            if (0..5).map(|k| s[i][j - k]).collect::<Vec<_>>() == vec!['s', 'n', 'u', 'k', 'e'] {
                for k in 0..5 {
                    println!("{} {}", i + 1, j - k + 1);
                }
                return;
            }
            if (0..5).map(|k| s[i][j - k]).rev().collect::<Vec<_>>()
                == vec!['s', 'n', 'u', 'k', 'e']
            {
                for k in (0..5).rev() {
                    println!("{} {}", i + 1, j - k + 1);
                }
                return;
            }
        }
    }
    for i in 4..h {
        for j in 4..w {
            if (0..5).map(|k| s[i - k][j - k]).collect::<Vec<_>>() == vec!['s', 'n', 'u', 'k', 'e']
            {
                for k in 0..5 {
                    println!("{} {}", i - k + 1, j - k + 1);
                }
                return;
            }
            if (0..5).map(|k| s[i - k][j - k]).rev().collect::<Vec<_>>()
                == vec!['s', 'n', 'u', 'k', 'e']
            {
                for k in (0..5).rev() {
                    println!("{} {}", i - k + 1, j - k + 1);
                }
                return;
            }
        }
    }
    for i in 4..h {
        for j in 0..w - 4 {
            if (0..5).map(|k| s[i - k][j + k]).collect::<Vec<_>>() == vec!['s', 'n', 'u', 'k', 'e']
            {
                for k in 0..5 {
                    println!("{} {}", i - k + 1, j + k + 1);
                }
                return;
            }
            if (0..5).map(|k| s[i - k][j + k]).rev().collect::<Vec<_>>()
                == vec!['s', 'n', 'u', 'k', 'e']
            {
                for k in (0..5).rev() {
                    println!("{} {}", i - k + 1, j + k + 1);
                }
                return;
            }
        }
    }
}
