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
        q: usize,
        mut a: [usize; n],
        txy: [(usize, usize, usize); q],
    }
    let m = 30;
    let mut bit = vec![];
    for _ in 0..m {
        bit.push(FenwickTree::new(n, 0));
    }
    for i in 0..n {
        for j in 0..m {
            if a[i] & 1 << j > 0 {
                bit[j].add(i, 1);
            }
        }
    }
    for &(ti, xi, yi) in txy.iter() {
        if ti == 1 {
            let i = xi - 1;
            for j in 0..m {
                if (a[i] & yi) & 1 << j > 0 {
                    bit[j].add(i, -1);
                }
                if a[i] & 1 << j == 0 && yi & 1 << j > 0 {
                    bit[j].add(i, 1);
                }
            }
            a[i] ^= yi;
        } else {
            let mut ans = 0;
            for j in 0..m {
                if bit[j].sum(xi - 1..yi) % 2 == 1 {
                    ans |= 1 << j;
                }
            }
            println!("{}", ans);
        }
    }
}
