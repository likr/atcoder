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
        mut h: [usize; n],
    }
    let mut count = 0;
    while h.iter().any(|&hi| hi > 0) {
        let mut l = 0;
        while h[l] == 0 {
            l += 1;
        }
        let mut r = l + 1;
        while r < n && h[r] > 0 {
            r += 1;
        }
        // eprintln!("{} {}", l, r);
        let mut min_h = h[l];
        for i in l..r {
            min_h = min(min_h, h[i]);
        }
        for i in l..r {
            h[i] -= min_h;
        }
        count += min_h;
    }
    println!("{}", count);
}
