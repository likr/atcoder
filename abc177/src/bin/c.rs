use ac_library::ModInt1000000007;
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
        a: [usize; n],
    }
    let f = ModInt1000000007::new;
    let mut acc = vec![f(a[0]); n];
    for i in 1..n {
        acc[i] = acc[i - 1] + f(a[i]);
    }
    let mut ans = f(0);
    for j in 1..n {
        ans = ans + f(a[j]) * acc[j - 1];
    }
    println!("{}", ans);
}
