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
        n: i64,
        m: i64,
    }
    if n == 1 && m == 0 {
        println!("1 2");
    } else if m < 0 || n - (m + 2) < 0 {
        println!("-1");
    } else {
        let mut lr = vec![];
        let x = n - (m + 2);
        for i in 0..x {
            lr.push((2 * i + 1, 2 * i + 2));
        }
        lr.push((2 * x + 1, 2 * x + 2 + (m + 1) * 2));
        for i in 0..=m.abs() {
            lr.push((2 * x + 2 + 2 * i, 2 * x + 3 + 2 * i));
        }

        debug!(lr);
        assert!(lr.len() as i64 == n);
        lr.sort_by_key(|&(_, ri)| ri);
        let mut a = 1;
        let mut last = 0;
        for i in 1..n as usize {
            if lr[last].1 < lr[i].0 {
                last = i;
                a += 1;
            }
        }
        lr.sort_by_key(|&(li, _)| li);
        let mut b = 1;
        let mut last = 0;
        for i in 1..n as usize {
            if lr[last].1 < lr[i].0 {
                last = i;
                b += 1;
            }
        }
        debug!(a, b);
        assert!(a - b == m);
        for &(li, ri) in &lr {
            println!("{} {}", li, ri);
        }
    }
}
