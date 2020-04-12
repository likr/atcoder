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
        k: usize,
        c: isize,
        mut s: Chars,
    }
    let mut last = -(INF as isize);
    let mut fastest = vec![];
    for i in 0..n {
        if s[i] == 'o' && i as isize > last + c && fastest.len() < k {
            fastest.push(i);
            last = i as isize;
        }
    }
    s.reverse();
    let mut last = -(INF as isize);
    let mut slowest = vec![];
    for i in 0..n {
        if s[i] == 'o' && i as isize > last + c && slowest.len() < k {
            slowest.push(n - 1 - i);
            last = i as isize;
        }
    }
    slowest.reverse();
    for i in 0..k {
        if fastest[i] == slowest[i] {
            println!("{}", fastest[i] + 1);
        }
    }
}
