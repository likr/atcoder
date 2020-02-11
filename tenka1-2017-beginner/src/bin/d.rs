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
        k: usize,
        ab: [(usize, u64); n],
    }
    let ab = ab.iter().filter(|&(ai, _)| *ai <= k).collect::<Vec<_>>();
    let mut m = 0;
    let mut k2 = k;
    while k2 > 0 {
        m += 1;
        k2 /= 2;
    }
    let mut result = 0u64;
    let x = 2usize.pow(m as u32) - 1;
    if x == k {
        let mut s = 0;
        for &(_, bi) in &ab {
            s += bi;
        }
        result = s;
    }
    eprintln!("{} {:b}", m, x);
    for i in 0..m {
        let y = x ^ (1 << i);
        let mut s = 0;
        let mut z = 0;
        for &(ai, bi) in &ab {
            if y | ai == y {
                z |= ai;
                s += bi;
            }
        }
        eprintln!("{:b} {}", y, s);
    }
    println!("{}", result);
}
