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

fn modpow(x: usize, y: usize, m: usize) -> usize {
    let mut result = 1;
    let mut a = x;
    let mut b = y;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % m;
        }
        a = a * a % m;
        b /= 2;
    }
    result
}

fn win(x: char, y: char) -> char {
    if x == y {
        x
    } else if x == 'R' {
        if y == 'P' {
            y
        } else {
            x
        }
    } else if x == 'P' {
        if y == 'R' {
            x
        } else {
            y
        }
    } else {
        if y == 'P' {
            x
        } else {
            y
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut dp = vec![HashMap::new(); k + 1];
    for i in 0..n {
        let j = (i + 1) % n;
        dp[0].insert((i, j), s[i]);
    }
    for d in 1..=k {
        for i in 0..n {
            let j = (i + modpow(2, d, n)) % n;
            let m = (i + modpow(2, d - 1, n)) % n;
            let l = dp[d - 1][&(i, m)];
            let r = dp[d - 1][&(m, j)];
            dp[d].insert((i, j), win(l, r));
        }
        debug!(d, dp[d]);
    }
    let i = 0;
    let j = modpow(2, k, n);
    println!("{}", dp[k][&(i, j)]);
}
