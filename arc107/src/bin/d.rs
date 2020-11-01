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
const M: usize = 998244353;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn rec(target: usize, remains: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // debug!(target, max_num, remains);
    if let Some(&result) = cache.get(&(target, remains)) {
        result
    } else {
        let result = if remains < target {
            0
        } else if target == 0 && remains == 0 {
            1
        } else if target == 0 || remains == 0 {
            0
        } else {
            (rec(target - 1, remains - 1, cache) + rec(2 * target, remains, cache)) % M
        };
        cache.insert((target, remains), result);
        result
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut cache = HashMap::new();
    println!("{}", rec(k, n, &mut cache));
}
