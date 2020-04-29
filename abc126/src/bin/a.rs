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
        _n: usize,
        k: Usize1,
        mut s: Chars,
    }
    s[k] = (s[k] as usize + 'a' as usize - 'A' as usize) as u8 as char;
    println!("{}", s.iter().collect::<String>());
}
