use ac_library::*;
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
        m: usize,
    }
    let f = ModInt998244353::new;
    let mut ans = f(0);
    for i in 0..60 {
        debug!(i, ans);
        if m & 1 << i > 0 {
            let c = 1 << (i + 1);
            let d = 1 << i;
            ans += f(d) * f((n + 1) / c);
            if (n + 1) % c > d {
                ans += f((n + 1) % c - d);
            }
        }
    }
    println!("{}", ans);
}
