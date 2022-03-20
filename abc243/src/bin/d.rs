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
        mut x: usize,
        s: Chars,
    }
    let mut stack = vec![];
    for i in 0..n {
        if s[i] == 'U' {
            if let Some(s0) = stack.pop() {
                if s0 == 'U' {
                    stack.push(s0);
                    stack.push(s[i]);
                }
            } else {
                stack.push(s[i]);
            }
        } else {
            stack.push(s[i]);
        }
    }
    for &si in stack.iter() {
        if si == 'U' {
            x = x / 2;
        }
        if si == 'L' {
            x = x * 2;
        }
        if si == 'R' {
            x = x * 2 + 1;
        }
    }
    println!("{}", x);
}
