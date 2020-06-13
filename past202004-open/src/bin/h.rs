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
        m: usize,
        a: [Chars; n],
    }
    let mut b = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            b[i][j] = if a[i][j] == 'S' {
                0
            } else if a[i][j] == 'G' {
                10
            } else {
                a[i][j] as usize - '0' as usize
            };
        }
    }
    let mut cells = vec![vec![]; 11];
    for i in 0..n {
        for j in 0..m {
            cells[b[i][j]].push((i, j));
        }
    }
    let mut d = vec![vec![INF; m]; n];
    d[cells[0][0].0][cells[0][0].1] = 0;
    for k in 1..=10 {
        for &(i1, j1) in &cells[k] {
            for &(i2, j2) in &cells[k - 1] {
                let e = (i1 as i64 - i2 as i64).abs() + (j1 as i64 - j2 as i64).abs();
                d[i1][j1] = min(d[i1][j1], d[i2][j2] + e as usize);
            }
        }
    }
    let result = d[cells[10][0].0][cells[10][0].1];
    if result == INF {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
