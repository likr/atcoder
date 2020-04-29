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
        c: [Chars; h],
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", c[i][j]);
        }
        println!("");
        for j in 0..w {
            print!("{}", c[i][j]);
        }
        println!("");
    }
}
