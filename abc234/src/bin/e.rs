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
    let mut nums = vec![];
    nums.push(111111111111111111);
    for diff in -9..=9 {
        for s in 0..=9 {
            if diff == 0 && s == 0 {
                continue;
            }
            let mut n = 0;
            let mut b = 1;
            let mut c = s;
            while n < 111111111111111111 && (0 <= c && c <= 9) {
                n += c * b;
                nums.push(n);
                b *= 10;
                c += diff;
            }
        }
    }
    nums.sort();
    for i in 0..nums.len() {
        if nums[i] >= x {
            println!("{}", nums[i]);
            break;
        }
    }
}
