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
    let chars = "ABCDEF";
    let mut nums = vec![];
    for c in chars.chars() {
        nums.push(format!("{}", s.iter().filter(|&&si| c == si).count()));
    }
    println!("{}", nums.join(" "));
}
