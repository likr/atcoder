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
        n: Chars,
    }
    let mut sum = 0usize;
    for &c in &n {
        sum += c as usize - '0' as usize;
    }
    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
