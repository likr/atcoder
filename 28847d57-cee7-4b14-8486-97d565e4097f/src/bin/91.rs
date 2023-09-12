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
        p: usize,
    }
    let f = ModInt998244353::new;
    let mut fact = vec![f(1); n + 1];
    for i in 1..=n {
        fact[i] = f(i) * fact[i - 1];
    }
    let q = f(100 - p) / f(100);
    let p = f(p) / f(100);
    let mut s1 = f(0);
    let mut s2 = f(0);
    let mut t = f(0);
    for i in 0..=n {
        let j = (n - i + 1) / 2;
        let a = p.pow(j as u64);
        let b = q.pow(i as u64);
        if 2 * j + i == n {
            let c = fact[i + j] / fact[i] / fact[j] * a * b;
            s1 += f(i + j) * c;
            t += c;
        } else {
            let c = fact[i + j - 1] / fact[i] / fact[j - 1] * a * b;
            s2 += f(i + j) * c;
            t += c;
        }
    }
    println!("{}", (s1 + s2) / t);
}
