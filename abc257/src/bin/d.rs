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
        xyp: [(i64, i64, i64); n],
    }
    let mut l = 0;
    let mut h = 10000000000;
    while h - l > 1 {
        let m = (l + h) / 2;
        let mut distance = vec![vec![INF; n]; n];
        for i in 0..n {
            let (xi, yi, pi) = xyp[i];
            distance[i][i] = 0;
            for j in 0..n {
                if i == j {
                    continue;
                }
                let (xj, yj, _) = xyp[j];
                let d = (xi - xj).abs() + (yi - yj).abs();
                if pi * m >= d {
                    distance[i][j] = 1;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    distance[i][j] = min(distance[i][j], distance[i][k] + distance[k][j]);
                }
            }
        }
        if (0..n).any(|i| (0..n).all(|j| distance[i][j] != INF)) {
            h = m;
        } else {
            l = m;
        }
    }
    println!("{}", h);
}
