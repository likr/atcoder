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
        k: usize,
        a: [i64; n],
    }
    let mut count = HashMap::new();
    for i in 0..n {
        count.insert(a[i], 0);
    }
    let mut used = HashSet::new();
    let mut j = 0;
    let mut result = 0;
    for i in 0..n {
        while j < n && !(used.len() == k && !used.contains(&a[j])) {
            used.insert(a[j]);
            count.insert(a[j], count[&a[j]] + 1);
            j += 1;
        }
        result = max(result, j - i);
        count.insert(a[i], count[&a[i]] - 1);
        if count[&a[i]] == 0 {
            used.remove(&a[i]);
        }
    }
    println!("{}", result);
}
