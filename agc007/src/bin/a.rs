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
    let mut impossible = false;
    for i in 1..h {
        for j in 1..w {
            if a[i - 1][j - 1] == '#' {
                if a[i][j - 1] == '#' && a[i - 1][j] == '#' {
                    impossible = true;
                }
            }
            if a[i][j] == '#' {
                if a[i - 1][j] == '.' && a[i][j - 1] == '.' {
                    impossible = true;
                }
            }
        }
    }
    if impossible {
        println!("Impossible");
    } else {
        println!("Possible");
    }
}
