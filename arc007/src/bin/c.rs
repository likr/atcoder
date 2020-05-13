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
        c: Chars,
    }
    let n = c.len();
    let mut start = 0;
    while start < n && c[start] != 'o' {
        start += 1;
    }

    let mut t = vec![];
    for i in start..n {
        t.push(c[i]);
    }
    for i in 0..start {
        t.push(c[i]);
    }

    let mut result = 1;
    while (0..n).any(|i| t[i] == 'x') {
        let mut i = 0;
        while i < n && t[i] == 'o' {
            i += 1;
        }
        for j in 0.. {
            if i + j >= n {
                break;
            }
            if c[start + j] == 'o' {
                t[i + j] = 'o'
            }
        }
        result += 1;
    }
    println!("{}", result);
}
