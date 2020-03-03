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
        s: [Chars; h],
    }

    let d = [(1, 0), (1, 2), (0, 1), (2, 1)];
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if s[i][j] == '#' {
                if d.iter().all(|&(di, dj)| s[i + di - 1][j + dj - 1] == '.') {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
