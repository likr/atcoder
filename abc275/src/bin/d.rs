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

fn f(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&v) = cache.get(&n) {
        v
    } else {
        if n == 0 {
            1
        } else {
            let result = f(n / 2, cache) + f(n / 3, cache);
            cache.insert(n, result);
            result
        }
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut cache = HashMap::new();
    println!("{}", f(n, &mut cache));
}
