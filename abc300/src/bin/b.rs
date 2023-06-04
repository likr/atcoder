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
        a: [Chars; h],
        b: [Chars; h],
    }
    for i in 0..h {
        for j in 0..w {
            let mut ok = true;
            for i2 in 0..h {
                for j2 in 0..w {
                    if a[i2][j2] != b[(i2 + i) % h][(j2 + j) % w] {
                        ok = false;
                    }
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
