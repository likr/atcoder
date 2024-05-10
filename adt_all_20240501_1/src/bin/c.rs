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
        x: [[Usize1]; m],
    }
    let mut mat = vec![vec![0; n]; n];
    for xi in x.iter() {
        let k = xi.len();
        for j1 in 0..k {
            for j2 in 0..j1 {
                mat[xi[j1]][xi[j2]] += 1;
                mat[xi[j2]][xi[j1]] += 1;
            }
        }
    }
    for i in 0..n {
        for j in 0..i {
            if mat[i][j] == 0 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
