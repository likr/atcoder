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
        q: usize,
        a: [usize; n],
        xk: [(usize, Usize1); q],
    }
    let mut indices = HashMap::new();
    for i in 0..n {
        indices.entry(a[i]).or_insert(vec![]).push(i + 1);
    }
    for &(xi, ki) in xk.iter() {
        if indices.contains_key(&xi) && ki < indices[&xi].len() {
            println!("{}", indices[&xi][ki]);
        } else {
            println!("-1");
        }
    }
}
