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
        s: Chars,
    }
    let mut w_count = vec![0; n];
    for i in 1..n {
        w_count[i] = w_count[i - 1];
        if s[i - 1] == 'W' {
            w_count[i] += 1;
        }
    }
    let mut e_count = vec![0; n];
    for i in (0..n - 1).rev() {
        e_count[i] = e_count[i + 1];
        if s[i + 1] == 'E' {
            e_count[i] += 1;
        }
    }
    // eprintln!("{:?}", w_count);
    // eprintln!("{:?}", e_count);
    println!("{}", (0..n).map(|i| w_count[i] + e_count[i]).min().unwrap());
}
