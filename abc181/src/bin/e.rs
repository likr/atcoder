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
const INF: i64 = std::i64::MAX / 4;
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
        m: usize,
        mut h: [i64; n],
        mut w: [i64; m],
    }
    h.sort();
    w.sort();

    let mut diff = vec![0; n - 1];
    for i in 1..n {
        diff[i - 1] = (h[i - 1] - h[i]).abs();
    }
    let mut acc_odd = vec![0];
    let mut acc_even = vec![0];
    for i in 1..n {
        if i % 2 == 0 {
            acc_odd.push(acc_odd[acc_odd.len() - 1] + diff[i - 1]);
        } else {
            acc_even.push(acc_even[acc_even.len() - 1] + diff[i - 1]);
        }
    }
    debug!(acc_odd);
    debug!(acc_even);

    let mut result = INF;
    for i in (0..n).step_by(2) {
        let left = i;
        let target = h[i];
        let k = w.lower_bound(&target);
        let d = if k == 0 {
            (target - w[k]).abs()
        } else if k == m {
            (target - w[k - 1]).abs()
        } else {
            min((target - w[k]).abs(), (target - w[k - 1]).abs())
        };
        result = min(
            result,
            d + acc_even[left / 2] + acc_odd[n / 2] - acc_odd[left / 2],
        );
    }
    println!("{}", result);
}
