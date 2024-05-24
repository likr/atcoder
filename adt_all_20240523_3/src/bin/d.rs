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
        pf: [(usize, [Usize1]); n],
    }
    let mut p = vec![];
    let mut f = vec![vec![false; m]; n];
    for i in 0..n {
        p.push(pf[i].0);
        for &j in pf[i].1.iter() {
            f[i][j] = true;
        }
    }
    for i in 0..n {
        for j in 0..n {
            if p[i] < p[j] {
                continue;
            }
            if (0..m).filter(|&k| f[i][k]).any(|k| !f[j][k]) {
                continue;
            }
            if p[i] > p[j] || (0..m).filter(|&k| f[j][k]).any(|k| !f[i][k]) {
                debug!(i, j);
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
