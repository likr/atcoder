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
        mut s: (Usize1, Usize1),
        c: [Chars; h],
        x: Chars,
    }
    for &xi in x.iter() {
        let (di, dj) = match xi {
            'L' => (0, !0),
            'R' => (0, 1),
            'U' => (!0, 0),
            _ => (1, 0),
        };
        let i2 = s.0.wrapping_add(di);
        let j2 = s.1.wrapping_add(dj);
        if i2 < h && j2 < w && c[i2][j2] == '.' {
            s = (i2, j2);
        }
    }
    println!("{} {}", s.0 + 1, s.1 + 1);
}
