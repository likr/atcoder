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
    if s[0] >= 'a' {
        s[0] = (s[0] as usize - 'a' as usize + 'A' as usize) as u8 as char;
    }
    for i in 1..s.len() {
        if s[i] < 'a' {
            s[i] = (s[i] as usize - 'A' as usize + 'a' as usize) as u8 as char;
        }
    }
    println!("{}", s.iter().collect::<String>());
}
