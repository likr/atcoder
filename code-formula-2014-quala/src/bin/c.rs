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
    let mut used0 = HashSet::new();
    let mut result = vec![vec![]; n];
    for i in 0..n {
        let mut skip = 0;
        let mut used = HashSet::new();
        let mut j = 0;
        while j < k + skip {
            let i2 = j % n;
            let j2 = j / n;
            if i2 <= i {
                if used.contains(&a[i2][j2]) {
                    skip += 1;
                } else {
                    if !used0.contains(&a[i2][j2]) {
                        result[i].push(a[i2][j2]);
                        used0.insert(a[i2][j2]);
                    }
                    used.insert(a[i2][j2]);
                }
            }
            j += 1;
        }
    }
    for i in 0..n {
        result[i].sort();
        println!(
            "{}",
            result[i]
                .iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
