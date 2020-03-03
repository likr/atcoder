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
        a: [Chars; h],
    }
    let mut h_mask = vec![true; h];
    for i in 0..h {
        h_mask[i] = (0..w).any(|j| a[i][j] == '#');
    }
    let mut w_mask = vec![true; w];
    for j in 0..w {
        w_mask[j] = (0..h).any(|i| a[i][j] == '#');
    }
    for i in 0..h {
        for j in 0..w {
            if h_mask[i] && w_mask[j] {
                print!("{}", a[i][j]);
            }
        }
        println!("");
    }
}
