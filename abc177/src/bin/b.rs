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
        t: Chars,
    }
    let mut result = INF;
    for i in 0..=s.len() - t.len() {
        let mut count = 0;
        for j in 0..t.len() {
            if s[i + j] != t[j] {
                count += 1;
            }
        }
        result = min(result, count);
    }
    println!("{}", result);
}
