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
        c: i64,
        abc: [(usize, usize, i64); n],
    }
    let mut events = BinaryHeap::new();
    for &(ai, bi, ci) in abc.iter() {
        events.push((Reverse(ai), ci));
        events.push((Reverse(bi + 1), -ci));
    }
    let mut result = 0i64;
    let mut total = 0;
    let mut day0 = 0;
    while let Some(&(Reverse(day), _)) = events.peek() {
        result += min(total, c) * (day - day0) as i64;
        while let Some(&(Reverse(d), ci)) = events.peek() {
            if d != day {
                break;
            }
            total += ci;
            events.pop().unwrap();
        }
        day0 = day;
    }
    println!("{}", result);
}
