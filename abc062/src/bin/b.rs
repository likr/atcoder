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
    let mut b = vec![vec!['#'; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            b[i + 1][j + 1] = a[i][j];
        }
    }
    for row in b {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}
