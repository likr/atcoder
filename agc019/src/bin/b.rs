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
        a: Chars,
    }
    let n = a.len();
    let mut c = vec![0usize; 26];
    for i in 0..n {
        c[a[i] as usize - 'a' as usize] += 1;
    }
    // eprintln!("{:?}", c);
    let mut result = n * (n - 1) / 2 + 1;
    for i in 0..26 {
        if c[i] > 0 {
            result -= c[i] * (c[i] - 1) / 2;
        }
    }
    println!("{}", result);
}
