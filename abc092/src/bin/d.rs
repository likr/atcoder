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
        a: usize,
        b: usize,
    }
    let (h, w) = (99, 99);
    let (c1, c2) = if a > b { ('.', '#') } else { ('#', '.') };
    let (a, b) = if a > b { (a, b) } else { (b, a) };

    let mut s = vec![vec![c2; w]; h];
    for k in 0..a {
        let i = k / 25;
        let j = k % 25;
        for i2 in 0..3 {
            for j2 in 0..3 {
                s[4 * i + i2][4 * j + j2] = c1;
            }
        }
    }
    for k in 0..b - 1 {
        let i = k / 25;
        let j = k % 25;
        s[4 * i + 1][4 * j + 1] = c2;
    }

    println!("{} {}", h, w);
    for i in 0..h {
        for j in 0..w {
            print!("{}", s[i][j]);
        }
        println!("");
    }
}
