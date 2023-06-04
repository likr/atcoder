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
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut result = vec![];
    for d in 1..=min(h, w) {
        let mut count = 0;
        for i in d..h - d {
            for j in d..w - d {
                let mut ok = true;
                if (0..=d).any(|k| c[i - k][j - k] == '.') {
                    ok = false;
                }
                if (0..=d).any(|k| c[i - k][j + k] == '.') {
                    ok = false;
                }
                if (0..=d).any(|k| c[i + k][j - k] == '.') {
                    ok = false;
                }
                if (0..=d).any(|k| c[i + k][j + k] == '.') {
                    ok = false;
                }
                if ok {
                    count += 1;
                }
            }
        }
        result.push(count);
    }
    let mut acc = 0;
    for i in (0..result.len() - 1).rev() {
        result[i] -= acc;
        acc += result[i];
    }
    println!(
        "{}",
        result
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
