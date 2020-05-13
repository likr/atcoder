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
        x: Chars,
    }
    let n = x.len();
    let mut stack = 0;
    let mut result = 0;
    for i in 0..n {
        if x[i] == 'S' {
            stack += 1;
        } else {
            if stack == 0 {
                result += 1;
            } else {
                stack -= 1;
            }
        }
    }
    println!("{}", result + stack);
}
