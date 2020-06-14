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
        h: usize,
        w: usize,
        n: usize,
        r: usize,
        c: usize,
        s: Chars,
        t: Chars,
    }
    let mut p = c;
    for i in 0..n {
        if s[i] == 'R' {
            p += 1;
        }
        if p > w {
            println!("NO");
            return;
        }
        if t[i] == 'L' && p > 1 {
            p -= 1;
        }
    }
    let mut p = c;
    for i in 0..n {
        if s[i] == 'L' {
            p -= 1;
        }
        if p == 0 {
            println!("NO");
            return;
        }
        if t[i] == 'R' && p < w {
            p += 1;
        }
    }
    let mut p = r;
    for i in 0..n {
        if s[i] == 'D' {
            p += 1;
        }
        if p > h {
            println!("NO");
            return;
        }
        if t[i] == 'U' && p > 1 {
            p -= 1;
        }
    }
    let mut p = r;
    for i in 0..n {
        if s[i] == 'U' {
            p -= 1;
        }
        if p == 0 {
            println!("NO");
            return;
        }
        if t[i] == 'D' && p < h {
            p += 1;
        }
    }
    println!("YES");
}
