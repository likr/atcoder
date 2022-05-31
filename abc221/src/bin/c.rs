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
        mut n: usize,
    }
    let mut d = vec![];
    while n > 0 {
        d.push(n % 10);
        n /= 10;
    }
    let m = d.len();
    let mut result = 0;
    for x in 0..2usize.pow(m as u32) {
        let mut a = vec![];
        let mut b = vec![];
        for i in 0..m {
            if x & 1 << i == 0 {
                a.push(d[i]);
            } else {
                b.push(d[i]);
            }
        }
        a.sort();
        a.reverse();
        let mut a_val = 0;
        for &ai in a.iter() {
            a_val = 10 * a_val + ai;
        }
        b.sort();
        b.reverse();
        let mut b_val = 0;
        for &bi in b.iter() {
            b_val = 10 * b_val + bi;
        }
        result = max(result, a_val * b_val);
    }
    println!("{}", result);
}
