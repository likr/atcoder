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
        a: usize,
        b: usize,
    }
    let mut s = vec![vec!['0'; w]; h];
    for i in 0..h - b {
        for j in 0..w - a {
            s[i][j] = '1';
        }
    }
    for i in h - b..h {
        for j in w - a..w {
            s[i][j] = '1';
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", s[i][j]);
        }
        println!("");
    }
}
