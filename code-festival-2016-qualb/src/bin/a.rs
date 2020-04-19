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
        s: Chars,
    }
    let t = "CODEFESTIVAL2016".chars().collect::<Vec<_>>();
    let mut count = 0;
    for i in 0..16 {
        if s[i] != t[i] {
            count += 1;
        }
    }
    println!("{}", count);
}
