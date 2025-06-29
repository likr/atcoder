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
        n: usize,
        ab: [(Usize1, Usize1); n],
    }
    let mut row = vec![];
    let mut col = vec![];
    for &(ai, bi) in ab.iter() {
        row.push(ai);
        col.push(bi);
    }
    row.sort();
    row.dedup();
    col.sort();
    col.dedup();
    let row_map = row
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i + 1))
        .collect::<HashMap<_, _>>();
    let col_map = col
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i + 1))
        .collect::<HashMap<_, _>>();
    for &(ai, bi) in ab.iter() {
        println!("{} {}", row_map[&ai], col_map[&bi]);
    }
}
