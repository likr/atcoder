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

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        h: [usize; n],
    }
    let h_max = *h.iter().max().unwrap();
    let mut l = 1;
    let mut r = (h_max + b - 1) / b;
    while r - l > 1 {
        // eprintln!("{} {}", l, r);
        let m = (r + l) / 2;
        let mut c = 0;
        for i in 0..n {
            if h[i] > b * m {
                c += (h[i] - b * m + (a - b) - 1) / (a - b);
            }
        }
        // eprintln!(" {} {}", m, c);
        if c <= m {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r);
}
