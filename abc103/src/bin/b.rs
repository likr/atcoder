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
    let n = s.len();
    for i in 0..n {
        if (0..n).all(|j| s[j] == t[(i + j) % n]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
