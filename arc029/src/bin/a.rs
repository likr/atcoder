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
const INF: isize = std::isize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        t: [isize; n],
    }
    let mut result = INF;
    for x in 0..2usize.pow(n as u32) {
        let mut g1 = 0;
        let mut g2 = 0;
        for i in 0..n {
            if x & 1 << i > 0 {
                g1 += t[i];
            } else {
                g2 += t[i];
            }
        }
        result = min(result, max(g1, g2));
    }
    println!("{}", result);
}
