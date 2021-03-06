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
        mut s: Chars,
    }
    let n = s.len();
    let mut result = 0usize;
    let mut i = 0;
    let mut a = 0;
    while i < n {
        if s[i] == 'A' {
            a += 1;
        } else if i + 1 < n && s[i] == 'B' && s[i + 1] == 'C' {
            result += a;
            i += 1;
        } else {
            a = 0;
        }
        i += 1;
    }
    println!("{}", result);
}
