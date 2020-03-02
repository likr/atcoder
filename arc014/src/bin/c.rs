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
    let mut count = vec![0; 3];
    for i in 0..n {
        if s[i] == 'R' {
            count[0] += 1;
        } else if s[i] == 'G' {
            count[1] += 1;
        } else {
            count[2] += 1;
        }
    }
    println!("{}", count[0] % 2 + count[1] % 2 + count[2] % 2);
}
