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
        p: [[usize; w]; h]
    }
    let mut result = 0usize;
    let mut indices = vec![];
    for x in 1..2usize.pow(h as u32) {
        indices.clear();
        for i in 0..h {
            if x & 1 << i > 0 {
                indices.push(i);
            }
        }
        let mut count = HashMap::new();
        for j in 0..w {
            let v = p[indices[0]][j];
            if indices.iter().all(|&i| p[i][j] == v) {
                *count.entry(v).or_insert(0) += 1;
            }
        }
        if let Some(cols) = count.values().max() {
            result = max(result, cols * indices.len())
        }
    }
    println!("{}", result);
}
