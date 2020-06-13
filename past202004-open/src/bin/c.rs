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
        n: usize,
        mut s: [Chars; n],
    }
    for i in (0..n - 1).rev() {
        for j in 1..2 * n - 1 {
            if s[i][j] == '#'
                && (s[i + 1][j - 1] == 'X' || s[i + 1][j] == 'X' || s[i + 1][j + 1] == 'X')
            {
                s[i][j] = 'X';
            }
        }
    }
    for i in 0..n {
        for j in 0..2 * n - 1 {
            print!("{}", s[i][j]);
        }
        println!("");
    }
}
