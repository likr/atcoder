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

fn f(x: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(&v) = memo.get(&x) {
        v
    } else {
        if x == 1 {
            0
        } else {
            let v = x + f(x / 2, memo) + f(x - x / 2, memo);
            memo.insert(x, v);
            v
        }
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut memo = HashMap::new();
    println!("{}", f(n, &mut memo));
}
