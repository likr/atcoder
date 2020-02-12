use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let mut m = 0;
    let mut k2 = k;
    while k2 > 0 {
        m += 1;
        k2 /= 2;
    }
    let mut zeros = vec![0; m];
    let mut ones = vec![0; m];
    for j in 0..m {
        for &ai in &a {
            if ai & (1 << j) > 0 {
                ones[j] += 1;
            }
        }
        zeros[j] = n - ones[j];
    }
    let mut result: u64 = a.iter().map(|&ai| ai ^ k).sum();
    for i in 0..m {
        if (1 << i) & k > 0 {
            let mut mask = 0;
            for j in i + 1..m {
                if (1 << j) & k > 0 {
                    mask |= 1 << j;
                }
            }
            for j in 0..i {
                if zeros[j] > ones[j] {
                    mask |= 1 << j;
                }
            }
            let s = a.iter().map(|&ai| ai ^ mask).sum();
            // eprintln!("{} {:b} {}", i, mask, s);
            result = max(s, result);
        }
    }
    println!("{}", result);
}
