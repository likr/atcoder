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
        k: usize,
        s: Chars,
    }
    if s.len() > k {
        println!("{}...", (0..k).map(|i| s[i]).collect::<String>());
    } else {
        println!("{}", s.iter().collect::<String>());
    }
}
