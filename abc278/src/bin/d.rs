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
        a: [usize; n],
        q: usize,
    }
    let mut nums = HashMap::new();
    for i in 0..n {
        nums.insert(i, a[i]);
    }
    let mut base = 0;
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
            }
            nums.clear();
            base = x;
        } else if t == 2 {
            input! {
                i: Usize1,
                x: usize,
            }
            *nums.entry(i).or_insert(0) += x;
        } else {
            input! {
                i: Usize1,
            }
            if let Some(&v) = nums.get(&i) {
                println!("{}", base + v);
            } else {
                println!("{}", base);
            }
        }
    }
}
