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
        q: usize,
    }
    let mut heap = BinaryHeap::new();
    let mut a = vec![];
    let mut offset = 0;
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                a.push(x);
            }
            2 => {
                if let Some(Reverse(ai)) = heap.pop() {
                    println!("{}", ai);
                } else {
                    println!("{}", a[offset]);
                    offset += 1;
                }
            }
            _ => {
                for i in offset..a.len() {
                    heap.push(Reverse(a[i]));
                }
                a.clear();
                offset = 0;
            }
        }
    }
}
