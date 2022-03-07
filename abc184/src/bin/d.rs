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

fn solve(x: usize, y: usize, z: usize, cache: &mut HashMap<(usize, usize, usize), f64>) -> f64 {
    if let Some(&result) = cache.get(&(x, y, z)) {
        return result;
    }
    if x >= 100 || y >= 100 || z >= 100 {
        return 0.0;
    }
    let s = (x + y + z) as f64;
    let mut result = 0.0;
    result += x as f64 / s * (solve(x + 1, y, z, cache) + 1.0);
    result += y as f64 / s * (solve(x, y + 1, z, cache) + 1.0);
    result += z as f64 / s * (solve(x, y, z + 1, cache) + 1.0);
    cache.insert((x, y, z), result);
    result
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut cache = HashMap::new();
    println!("{}", solve(a, b, c, &mut cache));
}
