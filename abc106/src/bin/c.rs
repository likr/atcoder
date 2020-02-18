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
        k: usize,
    }
    let one_count = s.iter().take_while(|&&c| c == '1').count();
    if one_count < k {
        println!("{}", s[one_count]);
    } else {
        println!("1");
    }
}
