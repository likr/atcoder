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
        x: i64,
    }
    let mut nums = vec![111111111111111111i64];
    for d in 1..10 {
        nums.push(d);
        for _ in 2..18 {
            nums.push(nums.last().unwrap() * 10 + d);
        }
    }
    for d in 1..10 {
        for s in 1..10 {
            let mut a = d;
            loop {
                let e = a % 10 + s;
                if e >= 10 {
                    break;
                }
                a = a * 10 + e;
                nums.push(a);
            }
        }
        for s in 1..10 {
            let mut a = d;
            loop {
                let e = a % 10 - s;
                if e < 0 {
                    break;
                }
                a = a * 10 + e;
                nums.push(a);
            }
        }
    }

    nums.sort();
    nums.dedup();
    for &y in nums.iter() {
        if y >= x {
            println!("{}", y);
            return;
        }
    }
}
