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
        mut a: [usize; n],
    }
    let mut heap = BinaryHeap::new();
    let mut a_sum = 0;
    for i in 0..n {
        heap.push(Reverse(a[i]));
        a_sum += a[i];
    }
    if l > a_sum {
        heap.push(Reverse(l - a_sum));
    }
    let mut ans = 0;
    while heap.len() > 1 {
        let x = heap.pop().unwrap().0;
        let y = heap.pop().unwrap().0;
        heap.push(Reverse(x + y));
        ans += x + y;
    }
    println!("{}", ans);
}
