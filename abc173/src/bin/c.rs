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
        k: usize,
        c: [Chars; h],
    }
    let mut result = 0;
    for y in 0..2usize.pow(h as u32) {
        for x in 0..2usize.pow(w as u32) {
            let mut count = 0;
            for i in 0..h {
                for j in 0..w {
                    if 1 << i & y == 0 && 1 << j & x == 0 && c[i][j] == '#' {
                        count += 1;
                    }
                }
            }
            if count == k {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
