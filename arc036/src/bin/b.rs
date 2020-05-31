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
        h: [usize; n],
    }
    if n == 1 {
        println!("1");
        return;
    }
    let mut result = 0;
    for t in 0..n {
        if (t == 0 || h[t - 1] < h[t]) && (t == n - 1 || h[t] > h[t + 1]) {
            let mut s = t;
            while s > 0 && h[s - 1] < h[s] {
                s -= 1;
            }
            let mut u = t;
            while u < n - 1 && h[u] > h[u + 1] {
                u += 1;
            }
            eprintln!("{} {} {}", s, t, u);
            result = max(result, u - s + 1);
        }
    }
    println!("{}", result);
}
