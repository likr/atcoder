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
        b: usize,
        s: Chars,
    }
    if !(0..a).any(|i| s[i] == '-') && s[a] == '-' && !(0..b).any(|i| s[a + i + 1] == '-') {
        println!("Yes");
    } else {
        println!("No");
    }
}
