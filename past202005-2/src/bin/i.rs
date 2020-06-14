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
        q: usize,
    }
    let mut transpose = false;
    let mut row = (0..n).collect::<Vec<_>>();
    let mut col = (0..n).collect::<Vec<_>>();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                if transpose {
                    col.swap(a, b);
                } else {
                    row.swap(a, b);
                }
            }
            2 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                if transpose {
                    row.swap(a, b);
                } else {
                    col.swap(a, b);
                }
            }
            3 => {
                transpose = !transpose;
            }
            _ => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                if transpose {
                    println!("{}", row[b] * n + col[a]);
                } else {
                    println!("{}", row[a] * n + col[b]);
                }
            }
        }
        // eprintln!("{} {:?} {:?}", transpose, row, col);
    }
}
