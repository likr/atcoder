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
        k: usize,
        a: [Usize1; k],
        xy: [(f64, f64); n],
    }
    let mut ok = 1e10;
    let mut ng = 0.;
    let mut flag = vec![false; n];
    for _ in 0..50 {
        let m = (ok + ng) / 2.;
        for i in 0..n {
            flag[i] = false;
        }
        for i in 0..k {
            let (xi, yi) = xy[a[i]];
            for j in 0..n {
                let (xj, yj) = xy[j];
                let dx = xj - xi;
                let dy = yj - yi;
                let d = (dx * dx + dy * dy).sqrt();
                if d <= m {
                    flag[j] = true;
                }
            }
        }
        if flag.iter().all(|&b| b) {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}
