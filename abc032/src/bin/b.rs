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
        k: usize,
    }
    let n = s.len();
    let mut set = HashSet::new();
    for i in k..=n {
        set.insert(s[i - k..i].iter().collect::<String>());
    }
    println!("{}", set.len());
}
