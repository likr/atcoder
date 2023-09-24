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

fn height(n: usize) -> usize {
    let mut h = 0;
    let mut n = n;
    while n > 0 {
        h += 1;
        n /= 2;
    }
    h
}

fn query(n: usize, h: usize, x: usize, xh: usize, d: usize) -> usize {
    if d + xh > h {
        0
    } else if d + xh < h {
        2usize.pow(d as u32)
    } else {
        let p = 2usize.pow((h - xh) as u32);
        let l = x * p;
        let r = l + p - 1;
        if l > n {
            0
        } else {
            min(n, r) + 1 - l
        }
    }
}

fn main() {
    input! {
        t: usize,
        nxk: [(usize, usize, usize); t],
    }
    for &(n, x, k) in nxk.iter() {
        let h = height(n);
        let xh = height(x);
        let mut ans = query(n, h, x, xh, k);
        let mut u = x;
        let mut uh = xh;
        if 0 < k && k < xh {
            ans += 1;
        }
        for i in 0..xh - 1 {
            if u % 2 == 0 {
                if k >= i + 2 {
                    ans += query(n, h, u + 1, uh, k - i - 2);
                }
            } else {
                if k >= i + 2 {
                    ans += query(n, h, u - 1, uh, k - i - 2);
                }
            }
            u /= 2;
            uh -= 1;
        }
        println!("{}", ans);
    }
}
