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
        a: [[usize; k]; n],
    }
    let m = k / n;
    let mut used = HashSet::new();
    let mut skip = k % n;
    let mut result = vec![vec![]; n];
    for i in 0..n {
        for j in 0..m {
            if used.contains(&a[i][j]) {
                skip += 1;
            }
            if !used.contains(&a[i][j]) {
                result[i].push(format!("{}", a[i][j]));
                used.insert(a[i][j]);
            }
        }
        for l in 0..(skip + n - 1) / n {
            for j in 0..=i {
                if !used.contains(&a[j][m + l]) {
                    result[i].push(format!("{}", a[j][m + l]));
                    used.insert(a[j][m + l]);
                }
            }
        }
    }
    for i in 0..n {
        result[i].sort();
        println!("{}", result[i].join(" "));
    }
}
