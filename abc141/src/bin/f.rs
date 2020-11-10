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
    let m = 60;
    let mut bit_count = vec![0; m];
    for i in 0..n {
        let ai = a[i];
        for j in 0..m {
            if ai & 1 << j > 0 {
                bit_count[j] += 1;
            }
        }
    }
    debug!(bit_count);

    let mut b = a.clone();
    let mut offset = 0;
    for j in (0..m).rev() {
        if bit_count[j] > 0 && bit_count[j] % 2 == 0 {
            if let Some(k) = (offset..n).find(|&i| b[i] & 1 << j > 0) {
                for i in 0..n {
                    if i != k && b[i] & 1 << j > 0 {
                        b[i] ^= b[k];
                    }
                }
                b.swap(offset, k);
                offset += 1;
            }
        }
    }
    debug!(b);
    let mut s = 0;
    for i in 0..n {
        s ^= b[i];
    }
    debug!(s);

    let mut result = 0usize;
    for j in 0..m {
        if bit_count[j] % 2 == 0 {
            if s & 1 << j > 0 {
                result += 1 << (j + 1);
            }
        } else {
            result += 1 << j;
        }
    }
    println!("{}", result);
}
