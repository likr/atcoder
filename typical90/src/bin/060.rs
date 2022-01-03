use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
    let mut left = vec![INF; n];
    let mut left_index = vec![INF; n];
    for i in 0..n {
        let ai = a[i];
        let k = left.lower_bound(&ai);
        left[k] = min(left[k], ai);
        left_index[k] = min(left_index[k], i);
    }
    let mut left_length = left.lower_bound(&INF);
    let mut right_length = 0;
    let mut result = 0;
    let mut right = vec![INF; n];
    for i in (0..n).rev() {
        let ai = a[i];
        if i == left_index[left_length - 1] {
            left_length -= 1;
        }
        let k = right.lower_bound(&ai);
        right[k] = ai;
        right_length = max(right_length, k + 1);
        if left_length > 0 && right_length > 0 && left[left_length - 1] == right[right_length - 1] {
            result = max(result, left_length + right_length - 1);
        } else {
            result = max(result, left_length + right_length);
        }
    }
    println!("{}", result);
}
