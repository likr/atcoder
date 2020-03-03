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
    let mut result = 0;
    for i in 0..n {
        let mut s1 = HashSet::new();
        let mut s2 = HashSet::new();
        for j in 0..i {
            s1.insert(s[j]);
        }
        for j in i..n {
            s2.insert(s[j]);
        }
        result = max(result, s1.intersection(&s2).count());
    }
    println!("{}", result);
}
