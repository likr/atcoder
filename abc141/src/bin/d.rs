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
        m: usize,
        a: [usize; n],
    }
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((a[i], a[i], 0));
    }

    for _ in 0..m {
        let (p, q, c) = heap.pop().unwrap();
        if p == 0 {
            break;
        }
        heap.push((q / 2usize.pow(c + 1), q, c + 1));
    }

    let mut s = 0;
    while let Some((p, _, _)) = heap.pop() {
        s += p;
    }
    println!("{}", s);
}
