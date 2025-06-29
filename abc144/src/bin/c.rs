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
    }
    let mut f = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            f.push(i);
            f.push(n / i);
        }
    }
    f.sort();
    f.dedup();
    let mut ans = INF;
    for &fi in f.iter() {
        ans = min(ans, fi + n / fi - 2);
    }
    println!("{}", ans);
}
