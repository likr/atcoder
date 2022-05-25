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
        l: usize,
        a: [usize; n],
    }
    let mut heap = BinaryHeap::new();
    for &ai in a.iter() {
        heap.push(Reverse(ai));
    }
    let sum_a = a.iter().sum::<usize>();
    let mut result = 0usize;
    if sum_a < l {
        heap.push(Reverse(l - sum_a));
    }
    while heap.len() >= 2 {
        let Reverse(v1) = heap.pop().unwrap();
        let Reverse(v2) = heap.pop().unwrap();
        let v3 = v1 + v2;
        result += v3;
        heap.push(Reverse(v3));
    }
    println!("{}", result);
}
