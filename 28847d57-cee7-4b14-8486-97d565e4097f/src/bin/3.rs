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
        a: [usize; n],
    }
    let mut queue = VecDeque::new();
    for i in 0..k {
        queue.push_back(a[i]);
    }
    for i in k..n {
        let x = queue.pop_front().unwrap();
        if a[i] > x {
            println!("Yes");
        } else {
            println!("No");
        }
        queue.push_back(a[i]);
    }
}
