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
        s: Chars,
    }
    let mut count = vec![vec![0usize; 4]; n + 1];
    for i in 0..n {
        for j in 0..4 {
            count[i + 1][j] = count[i][j];
        }
        match s[i] {
            'A' => {
                count[i + 1][0] += 1;
            }
            'T' => {
                count[i + 1][1] += 1;
            }
            'C' => {
                count[i + 1][2] += 1;
            }
            'G' => {
                count[i + 1][3] += 1;
            }
            _ => {}
        }
    }
    debug!(count);
    let mut result = 0usize;
    for j in 1..=n {
        for i in 0..j {
            let a_count = count[j][0] - count[i][0];
            let t_count = count[j][1] - count[i][1];
            let c_count = count[j][2] - count[i][2];
            let g_count = count[j][3] - count[i][3];
            if a_count == t_count && c_count == g_count {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
