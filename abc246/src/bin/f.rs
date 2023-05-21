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
const M: usize = 998244353;

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
        l: usize,
        s: [Chars; n],
    }
    let s = s
        .into_iter()
        .map(|si| {
            si.into_iter()
                .map(|c| (c as u8 - 'a' as u8) as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut pow = vec![1; 26];
    for j in 1..26 {
        let mut x = l;
        let mut b = j + 1;
        while x > 0 {
            if x & 1 == 1 {
                pow[j] = pow[j] * b % M;
            }
            b = b * b % M;
            x /= 2;
        }
    }
    debug!(pow);
    let mut result = 0;
    for x in 1..(1 << n) {
        let mut bit_count = 0;
        let mut char_count = vec![0; 26];
        for i in 0..n {
            if x & 1 << i > 0 {
                bit_count += 1;
                for &c in s[i].iter() {
                    char_count[c] += 1;
                }
            }
        }
        let mut count = 0;
        for c in 0..26 {
            if char_count[c] == bit_count {
                count += 1;
            }
        }
        debug!(x, count);
        if count > 0 {
            if bit_count % 2 == 0 {
                result = (result + M - pow[count - 1]) % M;
            } else {
                result = (result + pow[count - 1]) % M;
            }
        }
    }
    println!("{}", result);
}
