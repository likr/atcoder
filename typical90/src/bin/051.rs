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
        k: usize,
        p: usize,
        a: [usize; n],
    }
    let m = min(n, 20);

    let mut items1 = vec![vec![]; n + 1];
    for x in 0..2usize.pow(m as u32) {
        let mut s = 0;
        let mut count = 0;
        for i in 0..m {
            if x & 1 << i > 0 {
                s += a[i];
                count += 1;
            }
        }
        items1[count].push(s);
    }

    if n == m {
        let mut result = 0;
        for &s in items1[k].iter() {
            if s <= p {
                result += 1;
            }
        }
        println!("{}", result);
        return;
    }

    let mut items2 = vec![vec![]; n + 1];
    for x in 0..2usize.pow((n - m) as u32) {
        let mut s = 0;
        let mut count = 0;
        for i in m..n {
            if x & 1 << (i - m) > 0 {
                s += a[i];
                count += 1;
            }
        }
        items2[count].push(s);
    }
    for i in 0..=n {
        items2[i].sort();
    }

    let mut result = 0;
    for i in 0..=k {
        for &s1 in items1[i].iter() {
            if s1 <= p {
                result += items2[k - i].upper_bound(&(p - s1));
            }
        }
    }
    println!("{}", result);
}
