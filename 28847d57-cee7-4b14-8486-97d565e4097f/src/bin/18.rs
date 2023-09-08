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
        mut s: Chars,
        q: usize,
    }
    let mut flip = false;
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                a: Usize1,
                b: Usize1,
            }
            if flip {
                s.swap((a + n) % (2 * n), (b + n) % (2 * n));
            } else {
                s.swap(a, b);
            }
        } else {
            input! {
                _a: usize,
                _b: usize,
            }
            flip = !flip;
        }
    }
    if flip {
        for i in 0..n {
            s.swap(i, i + n);
        }
    }
    println!("{}", s.iter().collect::<String>());
}
