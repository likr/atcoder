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
        xy: [(usize, usize); n],
    }
    let p = xy.iter().collect::<HashSet<_>>();
    let mut result = 0usize;
    for i in 0..n {
        for j in 0..i {
            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];
            if xi != xj && yi != yj && p.contains(&(xi, yj)) && p.contains(&(xj, yi)) {
                result += 1;
            }
        }
    }
    println!("{}", result / 2);
}
