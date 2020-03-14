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

fn digit(mut n: usize) -> usize {
    let mut c = 0;
    while n > 0 {
        n /= 10;
        c += 1;
    }
    c
}

fn main() {
    input! {
        n: usize,
    }
    let mut result = INF;
    for a in 1.. {
        if a * a > n {
            break;
        }
        if n % a == 0 {
            let b = n / a;
            result = min(result, max(digit(a), digit(b)));
        }
    }
    println!("{}", result);
}
