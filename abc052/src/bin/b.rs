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
    let mut max_x = 0isize;
    let mut x = 0isize;
    for i in 0..n {
        if s[i] == 'I' {
            x += 1;
            max_x = max(max_x, x);
        } else {
            x -= 1;
        }
    }
    println!("{}", max_x);
}
