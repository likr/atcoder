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
    s.reverse();
    let s = s
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    let n = s.len();
    let mut acc = vec![0; n + 1];
    let mut base = 1;
    for i in 0..n {
        acc[i + 1] = (acc[i] + base * s[i]) % 2019;
        base = base * 10 % 2019;
    }
    // eprintln!("{:?}", acc);
    let mut count = 0;
    for i in 0..2019 {
        let mut c = 0;
        for j in 0..=n {
            if acc[j] == i {
                c += 1;
            }
        }
        if c >= 2 {
            count += c * (c - 1) / 2;
        }
    }
    println!("{}", count);
}
