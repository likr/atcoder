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
    let mut t = vec![vec!['.'; w + 2]; h + 2];
    for i in 1..=h {
        for j in 1..=w {
            t[i][j] = s[i - 1][j - 1];
        }
    }
    let s = t;

    let mut t = vec![vec!['.'; w + 2]; h + 2];
    for i in 1..=h {
        for j in 1..=w {
            t[i][j] = '#';
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            if s[i][j] == '.' {
                for k in 0..3 {
                    for l in 0..3 {
                        t[i + k - 1][j + l - 1] = '.';
                    }
                }
            }
        }
    }
    for i in 1..=h {
        'outer: for j in 1..=w {
            if s[i][j] == '#' {
                for k in 0..3 {
                    for l in 0..3 {
                        if t[i + k - 1][j + l - 1] == '#' {
                            continue 'outer;
                        }
                    }
                }
                println!("impossible");
                return;
            }
        }
    }
    println!("possible");
    for i in 1..=h {
        for j in 1..=w {
            print!("{}", t[i][j]);
        }
        println!("");
    }
}
