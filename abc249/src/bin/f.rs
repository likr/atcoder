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
        k: usize,
        mut ty: [(usize, i64); n],
    }
    ty.reverse();
    ty.push((1, 0));
    ty.reverse();
    let n = ty.len();
    let mut result = -(INF as i64);
    let mut s = 0;
    let mut t1_count = 0;
    let mut heap = BinaryHeap::new();
    for i in (0..n).rev() {
        let (ti, yi) = ty[i];
        if ti == 1 {
            for j in i + 1..n {
                let (tj, yj) = ty[j];
                if tj == 1 {
                    break;
                }
                if yj >= 0 {
                    s += yj;
                } else {
                    heap.push(yj);
                }
            }
            while !heap.is_empty() && heap.len() + t1_count > k {
                s += heap.pop().unwrap();
            }
            result = max(result, yi + s);
            t1_count += 1;
            if k + 1 == t1_count {
                break;
            }
        }
    }
    println!("{}", result);
}
