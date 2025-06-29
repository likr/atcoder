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
        mut s: Chars,
        c: [usize; n],
    }
    let mut ans = INF;
    for _ in 0..2 {
        for i in 0..n {
            if s[i] == '0' {
                s[i] = '1';
            } else {
                s[i] = '0';
            }
        }

        let mut l = vec![0; n + 1];
        let mut r = vec![0; n + 1];
        for i in 0..n {
            if s[i] == ['0', '1'][i % 2] {
                l[i + 1] = c[i];
            } else {
                r[i] = c[i];
            }
        }
        debug!(l, r);
        for i in 0..n {
            l[i + 1] += l[i];
        }
        for i in (0..n).rev() {
            r[i] += r[i + 1];
        }
        debug!(l, r);
        for i in 1..n {
            ans = min(ans, l[i] + r[i]);
        }
    }
    println!("{}", ans);
}
