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
        a: usize,
        b: usize,
        c: usize,
    }
    let k = c.count_ones() as usize;
    for i in 0..=60 - k {
        for j in 0..=k {
            if i + j == a && i + k - j == b {
                let mut x = 0usize;
                let mut y = 0usize;
                let mut count1 = 0;
                let mut count2 = 0;
                for l in 0..60 {
                    if c & 1 << l > 0 {
                        if count1 < j {
                            x |= 1 << l;
                        } else {
                            y |= 1 << l;
                        }
                        count1 += 1;
                    } else {
                        if count2 < i {
                            x |= 1 << l;
                            y |= 1 << l;
                        }
                        count2 += 1;
                    }
                }
                assert!(x ^ y == c);
                assert!(x.count_ones() as usize == a);
                assert!(y.count_ones() as usize == b);
                println!("{} {}", x, y);
                return;
            }
        }
    }
    println!("-1");
}
