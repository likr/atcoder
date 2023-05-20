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
        mut xy: [(usize, usize); n],
    }
    xy.sort_by(|a, b| {
        if a.1 * (b.0 - 1) == b.1 * (a.0 - 1) {
            ((a.1 - 1) * b.0).cmp(&((b.1 - 1) * a.0))
        } else {
            (a.1 * (b.0 - 1)).cmp(&(b.1 * (a.0 - 1)))
        }
    });
    let mut current = 0;
    let mut result = 1;
    for i in 1..n {
        let (xi, yi) = xy[i];
        let (xj, yj) = xy[current];
        if (yi - 1) * (xj - 1) >= yj * xi {
            current = i;
            result += 1;
        }
    }
    println!("{}", result);
}
