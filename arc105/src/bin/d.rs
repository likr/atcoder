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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        }
        let mut count = HashMap::new();
        for i in 0..n {
            *count.entry(&a[i]).or_insert(0) += 1;
        }
        if n % 2 == 0 {
            if count.values().all(|&c| c % 2 == 0) {
                println!("Second");
            } else {
                println!("First");
            }
        } else {
            println!("Second");
        }
    }
}
