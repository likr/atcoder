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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn rec(a: &Vec<Vec<usize>>, sol: &mut Vec<(usize, usize)>, mask: usize) -> usize {
    let n = a.len();
    let mut result = 0;
    if mask == (1 << n) - 1 {
        for &(i, j) in sol.iter() {
            result ^= a[i][j];
        }
    } else {
        for i in 0..n {
            if mask & 1 << i == 0 {
                for j in i + 1..n {
                    if mask & 1 << j == 0 {
                        sol.push((i, j));
                        result = max(result, rec(a, sol, mask | (1 << i) | (1 << j)));
                        sol.pop();
                    }
                }
                break;
            }
        }
    }
    result
}

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![vec![0; 2 * n]; 2 * n];
    for i in 0..n * 2 {
        for j in i + 1..n * 2 {
            input! {
                ai: usize,
            }
            a[i][j] = ai;
            a[j][i] = ai;
        }
    }
    let mut sol = vec![];
    println!("{}", rec(&a, &mut sol, 0));
}
