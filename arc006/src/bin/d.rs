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

const B1: usize = 1009;
const B2: usize = 1007;
const M1: usize = 1000000007;
const M2: usize = 1000000009;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let a_pattern = vec![
        vec!['.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'o', '.', '.', '.'],
        vec!['.', '.', 'o', '.', 'o', '.', '.'],
        vec!['.', 'o', '.', '.', '.', 'o', '.'],
        vec!['.', 'o', 'o', 'o', 'o', 'o', '.'],
        vec!['.', 'o', '.', '.', '.', 'o', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.'],
    ];
    let b_pattern = vec![
        vec!['.', '.', '.', '.', '.', '.', '.'],
        vec!['.', 'o', 'o', 'o', 'o', '.', '.'],
        vec!['.', 'o', '.', '.', '.', 'o', '.'],
        vec!['.', 'o', 'o', 'o', 'o', '.', '.'],
        vec!['.', 'o', '.', '.', '.', 'o', '.'],
        vec!['.', 'o', 'o', 'o', 'o', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.'],
    ];
    let c_pattern = vec![
        vec!['.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', 'o', 'o', 'o', '.', '.'],
        vec!['.', 'o', '.', '.', '.', 'o', '.'],
        vec!['.', 'o', '.', '.', '.', '.', '.'],
        vec!['.', 'o', '.', '.', '.', 'o', '.'],
        vec!['.', '.', 'o', 'o', 'o', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.'],
    ];
    let mut pow1 = vec![vec![1; w + 1]; h + 1];
    let mut pow2 = vec![vec![1; w + 1]; h + 1];
    let mut hash1 = vec![vec![0; w + 1]; h + 1];
    let mut hash2 = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {}
    }
    println!("{:?}", a_pattern);
}
