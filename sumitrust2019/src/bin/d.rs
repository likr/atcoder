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
    let s = s
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    let mut result = HashSet::new();
    let mut one = HashSet::new();
    let mut ten = HashSet::new();
    for i in 0..n {
        for &v in &ten {
            result.insert(v * 10 + s[i]);
        }
        for &v in &one {
            ten.insert(v * 10 + s[i]);
        }
        one.insert(s[i]);
    }
    println!("{}", result.len());
}
