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
        rect: [(usize, usize, usize, usize); n],
    }
    let w = 1000;
    let h = 1000;
    let mut count = vec![vec![0i64; w + 1]; h + 1];
    for &(x1, y1, x2, y2) in rect.iter() {
        count[y1][x1] += 1;
        count[y1][x2] -= 1;
        count[y2][x1] -= 1;
        count[y2][x2] += 1;
    }
    let mut acc = vec![vec![0; w + 2]; h + 2];
    for i in 1..=h {
        for j in 1..=w {
            acc[i][j] = count[i - 1][j - 1] + acc[i - 1][j] + acc[i][j - 1] - acc[i - 1][j - 1];
        }
    }
    let mut result = vec![0; n + 1];
    for i in 1..=h {
        for j in 1..=w {
            if 0 <= acc[i][j] && acc[i][j] as usize <= n {
                result[acc[i][j] as usize] += 1;
            }
        }
    }
    for i in 1..=n {
        println!("{}", result[i]);
    }
}
