use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
    let mut p_count = 0i64;
    for &c in &s {
        if c == '+' {
            p_count += 1;
        }
    }
    let m_count = s.len() as i64 - p_count;
    println!("{}", p_count - m_count);
}
