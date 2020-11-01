use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        s: Chars,
        t: Chars,
    }
    let n = s.len();
    let m = t.len();
    let mut indices = vec![vec![]; 26];
    for i in 0..n {
        let c = s[i] as usize - 'a' as usize;
        indices[c].push(i);
    }
    let mut t_indices = vec![0; m];
    let mut last = n;
    for j in 0..m {
        let c = t[j] as usize - 'a' as usize;
        if indices[c].is_empty() {
            println!("-1");
            return;
        }
        let k = indices[c].upper_bound(&last);
        last = if k == indices[c].len() {
            indices[c][0]
        } else {
            indices[c][k]
        };
        t_indices[j] = last;
    }
    let mut cycles = 0;
    for i in 1..m {
        if t_indices[i - 1] >= t_indices[i] {
            cycles += 1;
        }
    }
    println!("{}", n * cycles + 1 + t_indices[m - 1]);
}
