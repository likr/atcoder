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
        n: usize,
        a: [usize; n],
        q: usize,
        lrx: [(Usize1, Usize1, usize); q],
    }
    let mut indices = HashMap::new();
    for i in 0..n {
        indices.entry(a[i]).or_insert(vec![]).push(i);
    }
    for &(li, ri, xi) in lrx.iter() {
        if let Some(is) = indices.get(&xi) {
            println!("{}", is.upper_bound(&ri) - is.lower_bound(&li));
        } else {
            println!("0");
        }
    }
}
