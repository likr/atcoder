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
        r: usize,
        c: usize,
        k: usize,
        s: [Chars; r],
    }
    let mut left = vec![vec![0; c]; r];
    let mut right = vec![vec![0; c]; r];
    for x in 0..r {
        for y in 0..c {
            if s[x][y] == 'o' {
                let mut l = 0;
                let mut j = y as isize;
                while 0 <= j && s[x][j as usize] == 'o' {
                    l += 1;
                    j -= 1;
                }
                left[x][y] = l;
                let mut r = 0;
                let mut j = y;
                while j < c && s[x][j] == 'o' {
                    r += 1;
                    j += 1;
                }
                right[x][y] = r;
            }
        }
    }
    let mut result = 0;
    for x in k - 1..=r - k {
        for y in k - 1..=c - k {
            if s[x][y] == 'o' {
                if (0..k).all(|i| {
                    left[x - i][y] >= k - i
                        && right[x - i][y] >= k - i
                        && left[x + i][y] >= k - i
                        && right[x + i][y] >= k - i
                }) {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
}
