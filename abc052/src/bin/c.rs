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
    }
    let mut div = vec![0; n + 1];
    for i in 1..=n {
        let mut m = i;
        let sqrt_m = (m as f64).sqrt() as usize;
        for j in 2..=sqrt_m {
            while m % j == 0 {
                m /= j;
                div[j] += 1;
            }
        }
        if m > 1 {
            div[m] += 1;
        }
    }
    // eprintln!("{:?}", div);
    let mut result = 1;
    for i in 2..=n {
        if div[i] > 0 {
            result = (result * (div[i] + 1)) % M;
        }
    }
    println!("{}", result);
}
