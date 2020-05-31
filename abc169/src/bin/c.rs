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
        a: usize,
        b: String,
    }
    let k = b.len() - b.chars().position(|c| c == '.').unwrap() - 1;
    let b = b
        .chars()
        .filter(|&c| c != '.')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let c = a * b;
    eprintln!("{}", k);
    println!("{}", c / 10usize.pow(k as u32));
}
