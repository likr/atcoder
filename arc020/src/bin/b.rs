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
        c: usize,
        a: [usize; n],
    }
    let mut result = INF;
    for i in 1..=10 {
        for j in 1..=10 {
            if i == j {
                continue;
            }
            let mut s = 0;
            for k in 0..n {
                if k % 2 == 0 {
                    if a[k] != i {
                        s += c;
                    }
                } else {
                    if a[k] != j {
                        s += c;
                    }
                }
            }
            result = min(result, s);
        }
    }
    println!("{}", result);
}
