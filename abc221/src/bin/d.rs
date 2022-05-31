use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*; #[allow(unused_imports)]
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
        ab: [(usize, usize); n],
    }
    let mut events = BinaryHeap::new();
    for &(ai, bi) in ab.iter() {
        events.push((Reverse(ai), 0));
        events.push((Reverse(ai + bi), 1));
    }
    let mut result = vec![0usize; n];
    let mut k = 0;
    while let Some(&(Reverse(i), _)) = events.peek() {
        while let Some(&(Reverse(j), f)) = events.peek() {
            if i == j {
                events.pop().unwrap();
                if f == 0 {
                    k += 1;
                } else {
                    k -= 1;
                }
            } else {
                if k > 0 {
                    result[k - 1] += j - i;
                }
                break;
            }
        }
    }
    let result = result.into_iter().map(|di| format!("{}", di)).collect::<Vec<_>>();
    println!("{}", result.join(" "));
}
