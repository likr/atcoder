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
        n: Usize1,
    }
    let mut words = vec![];
    for i in 0..s.len() {
        for j in 0..s.len() {
            words.push(format!("{}{}", s[i], s[j]));
        }
    }
    words.sort();
    println!("{}", words[n]);
}
