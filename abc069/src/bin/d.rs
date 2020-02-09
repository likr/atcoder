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
      n: usize,
      a: [usize; n],
    }
    let mut grid = vec![vec![0; w]; h];
    let mut i = 0;
    let mut j = 0;
    for (k, &ak) in a.iter().enumerate() {
        for _ in 0..ak {
            grid[i][j] = k + 1;
            if i % 2 == 0 {
                if j == w - 1 {
                    i += 1;
                } else {
                    j += 1;
                }
            } else {
                if j == 0 {
                    i += 1;
                } else {
                    j -= 1;
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{} ", grid[i][j]);
        }
        println!("");
    }
}
