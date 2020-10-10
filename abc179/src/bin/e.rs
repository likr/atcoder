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
        x: usize,
        m: usize,
    }
    if n < 10000000 {
        let mut res = x;
        let mut y = x;
        for _ in 1..n {
            y = y * y % m;
            res += y;
        }
        println!("{}", res);
        return;
    }

    let mut a = vec![];
    let mut a_set = HashSet::new();
    let mut y = x;
    while !a_set.contains(&y) {
        a.push(y);
        a_set.insert(y);
        y = y * y % m;
    }
    if n <= a.len() {
        let mut result = 0usize;
        for i in 0..n {
            result += a[i];
        }
        println!("{}", result);
        return;
    }
    debug!(a, y);
    debug!(a_set);
    let start = (0..a.len()).filter(|&i| a[i] == y).nth(0).unwrap();
    let cycle = a.len() - start;
    debug!(start, cycle);
    debug!(n / cycle, n % cycle);
    let mut result = 0usize;
    for i in 0..cycle {
        result += a[start + i];
    }
    result *= (n - start) / cycle;
    debug!(result);
    for i in 0..min(n, start) {
        result += a[i];
    }
    debug!(result);
    if n >= start {
        for i in 0..(n - start) % cycle {
            result += a[start + i];
        }
    }
    println!("{}", result);

    // let mut res = x;
    // let mut y = x;
    // for _ in 1..n {
    //     y = y * y % m;
    //     res += y;
    // }
    // debug!(result, res, max(result, res) - min(result, res));
}
