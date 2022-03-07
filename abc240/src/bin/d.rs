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
    }
    let mut count = 0usize;
    let mut stack = vec![];
    for i in 0..n {
        if let Some((x, c)) = stack.pop() {
            if a[i] == x {
                if c + 1 == x {
                    count -= c;
                } else {
                    stack.push((x, c + 1));
                    count += 1;
                }
            } else {
                stack.push((x, c));
                stack.push((a[i], 1));
                count += 1;
            }
        } else {
            stack.push((a[i], 1));
            count += 1;
        }
        println!("{}", count);
    }
}
