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

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
    #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        vw: [(Usize1, Usize1); n - 1],
    }
    let mut out_edges = vec![vec![]; n];
    let mut in_degree = vec![0; n];
    for &(mut vi, mut wi) in &vw {
        if vi > wi {
            std::mem::swap(&mut vi, &mut wi);
        }
        out_edges[vi].push(wi);
        in_degree[wi] += 1;
    }
    for i in 1..n {
        in_degree[i] += in_degree[i - 1];
    }

    let mut s = in_degree.iter().sum::<usize>();
    let mut result = 0usize;
    for vi in 0..n {
        debug!((n - vi) * (n - vi + 1) / 2, s);
        result += (n - vi) * (n - vi + 1) / 2 - s;
        for &wi in &out_edges[vi] {
            s -= n - wi;
        }
    }
    println!("{}", result);
}
