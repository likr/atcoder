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
        h: usize,
        w: usize,
        s: [[String; w]; h],
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == "snuke" {
                println!("{}{}", ('A' as usize + j) as u8 as char, i + 1);
            }
        }
    }
}
