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
    let mut queue = VecDeque::new();
    let mut heap = BinaryHeap::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
            }
            queue.push_back(x);
        } else if t == 2 {
            if let Some(Reverse(x)) = heap.pop() {
                println!("{}", x);
            } else if let Some(x) = queue.pop_front() {
                println!("{}", x);
            }
        } else {
            while let Some(x) = queue.pop_front() {
                heap.push(Reverse(x));
            }
        }
    }
}
