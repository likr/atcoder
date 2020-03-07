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
    let n = s.len();
    for i in 1..n {
        if s[i - 1] == s[i] {
            println!("{} {}", i, i + 1);
            return;
        }
    }
    for i in 2..n {
        if s[i - 2] == s[i] {
            println!("{} {}", i - 1, i + 1);
            return;
        }
    }
    println!("-1 -1");
}
