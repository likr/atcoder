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

    let mut result = 0usize;
    for i in 0..n {
        if s[i] == 'U' {
            result += n - i - 1;
            result += 2 * i;
        } else {
            result += 2 * (n - i - 1);
            result += i;
        }
    }

    println!("{}", result);
}
