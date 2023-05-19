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
        s: Chars,
    }
    let mut p = (0, 0);
    let mut points = HashSet::new();
    points.insert(p);
    for i in 0..n {
        match s[i] {
            'R' => {
                p.0 += 1;
            }
            'L' => {
                p.0 -= 1;
            }
            'U' => {
                p.1 += 1;
            }
            'D' => {
                p.1 -= 1;
            }
            _ => {}
        }
        points.insert(p);
    }
    if points.len() <= n {
        println!("Yes");
    } else {
        println!("No");
    }
}
