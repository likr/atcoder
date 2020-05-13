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
        _k: usize,
        mut s: [Chars; h],
    }
    let mut a = vec![vec![0; w]; h];
    let mut l = 1;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                a[i][j] = l;
                l += 1;
            }
        }
    }
    for i in 0..h {
        if (0..w).all(|j| s[i][j] == '.') {
            if i > 0 && a[i - 1][w - 1] != 0 {
                for j in 0..w {
                    a[i][j] = a[i - 1][j];
                }
            }
        } else {
            let mut l = 0;
            for j in 0..w {
                if a[i][j] == 0 {
                    a[i][j] = l;
                } else {
                    l = a[i][j];
                }
            }
            let mut l = 0;
            while a[i][l] == 0 {
                l += 1;
            }
            for j in 0..l {
                a[i][j] = a[i][l];
            }
        }
    }
    let mut l = 0;
    while a[l][0] == 0 {
        l += 1;
    }
    for i in 0..l {
        for j in 0..w {
            a[i][j] = a[l][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{} ", a[i][j]);
        }
        println!("");
    }
}
