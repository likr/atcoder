use permutohedron::LexicalPermutation;
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
        c: [usize; 9],
    }
    let mut count_all = 0usize;
    let mut count_ok = 0usize;
    let mut indices = (0..9).collect::<Vec<_>>();
    let x = vec![
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];
    loop {
        count_all += 1;
        if !x.iter().any(|&(i1, i2, i3)| {
            let mut is = vec![i1, i2, i3];
            is.sort_by_key(|&i| indices[i]);
            c[is[0]] == c[is[1]] && c[is[1]] != c[is[2]]
        }) {
            count_ok += 1;
        }
        if !indices.next_permutation() {
            break;
        }
    }
    println!("{}", count_ok as f64 / count_all as f64);
}
