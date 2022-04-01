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
        _h: usize,
        _w: usize,
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut x = HashSet::new();
    let mut y = HashSet::new();
    for &(ai, bi) in ab.iter() {
        x.insert(ai);
        y.insert(bi);
    }
    let mut x = x.iter().collect::<Vec<_>>();
    x.sort();
    let mut x_index = HashMap::new();
    for (i, &xi) in x.iter().enumerate() {
        x_index.insert(xi, i + 1);
    }
    let mut y = y.iter().collect::<Vec<_>>();
    y.sort();
    let mut y_index = HashMap::new();
    for (i, &yi) in y.iter().enumerate() {
        y_index.insert(yi, i + 1);
    }
    for &(ai, bi) in ab.iter() {
        println!("{} {}", x_index[&ai], y_index[&bi]);
    }
}
